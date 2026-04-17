use axum::{
    extract::State,
    http::{header, HeaderMap},
    response::{IntoResponse, Json, Response},
    http::StatusCode,
};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{PasswordHash, SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};
use crate::{AppState, cookies, db, error::AppError};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: Option<String>,
    pub display_name: String,
    pub spotify_linked: bool,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Response, AppError> {
    if req.username.contains('@') {
        return Err(AppError::BadRequest("Username cannot be an email address".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user = db::users::create_user(
        &state.db,
        &req.username,
        &password_hash,
        &req.display_name,
        req.email.as_deref(),
    )
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict("Username or email already taken".to_string());
            }
        }
        AppError::from(e)
    })?;

    let token = db::sessions::create_session(&state.db, user.id).await?;
    let body = UserResponse {
        id: user.id.to_string(),
        username: user.username,
        display_name: user.display_name,
        spotify_linked: user.spotify_refresh_token.is_some(),
    };

    let mut response = Json(body).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::session_cookie(&token).parse().unwrap(),
    );
    Ok(response)
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, AppError> {
    let user = db::users::find_by_username(&state.db, &req.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let stored_hash = user.password_hash.as_deref()
        .ok_or_else(|| AppError::Unauthorized("Account uses a different sign-in method".to_string()))?;

    let parsed = PasswordHash::new(stored_hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let token = db::sessions::create_session(&state.db, user.id).await?;
    let body = UserResponse {
        id: user.id.to_string(),
        username: user.username,
        display_name: user.display_name,
        spotify_linked: user.spotify_refresh_token.is_some(),
    };

    let mut response = Json(body).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::session_cookie(&token).parse().unwrap(),
    );
    Ok(response)
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if let Some(token) = cookies::extract_session_cookie(&headers) {
        db::sessions::delete_session(&state.db, &token).await?;
    }
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::clear_session_cookie().parse().unwrap(),
    );
    Ok(response)
}
