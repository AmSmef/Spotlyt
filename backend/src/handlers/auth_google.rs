use axum::{
    extract::{Query, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use crate::{auth, cookies, db, error::AppError, AppState};

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

pub async fn google_login() -> Result<Response, AppError> {
    let (auth_url, csrf_token) = auth::generate_google_auth_url()
        .map_err(AppError::Internal)?;

    let mut response = Redirect::to(&auth_url).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        format!(
            "google_csrf={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=600",
            csrf_token
        )
        .parse()
        .unwrap(),
    );
    Ok(response)
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let stored_csrf = cookies::extract_cookie(&headers, "google_csrf")
        .ok_or_else(|| AppError::BadRequest("Missing CSRF cookie".to_string()))?;

    if params.state != stored_csrf {
        return Err(AppError::Unauthorized("CSRF token mismatch".to_string()));
    }

    let google_user = auth::exchange_google_code(&params.code)
        .await
        .map_err(AppError::Internal)?;

    let user = db::users::find_or_create_by_google_id(
        &state.db,
        &google_user.google_id,
        &google_user.email,
        &google_user.display_name,
    )
    .await?;

    let token = db::sessions::create_session(&state.db, user.id).await?;

    let mut response = Redirect::to("http://127.0.0.1:5173?loggedin=true").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::session_cookie(&token).parse().unwrap(),
    );
    // Clear the google_csrf cookie
    response.headers_mut().append(
        header::SET_COOKIE,
        "google_csrf=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"
            .parse()
            .unwrap(),
    );
    Ok(response)
}
