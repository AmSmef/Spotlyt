use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
};
use serde::{Deserialize, Serialize};
use crate::{auth, db, error::AppError, AppState, AuthSession};

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

#[derive(Serialize)]
pub struct SpotifyStatusResponse {
    pub linked: bool,
}

/// Initiates the Spotify OAuth flow for an already-logged-in user.
/// Stores the CSRF token in the DB tied to the user so the callback
/// knows which user to write the refresh token to.
pub async fn spotify_link(
    State(state): State<AppState>,
    AuthSession { user }: AuthSession,
) -> Result<Response, AppError> {
    let (auth_url, csrf_token) = auth::generate_spotify_auth_url()
        .map_err(AppError::Internal)?;

    db::spotify_state::create_spotify_state(&state.db, user.id, &csrf_token)
        .await?;

    Ok(Redirect::to(&auth_url).into_response())
}

/// Handles the Spotify redirect after the user grants permissions.
/// Looks up which user initiated the flow, exchanges the code for tokens,
/// and persists the refresh token.
pub async fn spotify_callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> Result<Response, AppError> {
    let user_id = db::spotify_state::consume_spotify_state(&state.db, &params.state)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired OAuth state".to_string()))?;

    let tokens = auth::exchange_spotify_code(params.code)
        .await
        .map_err(AppError::Internal)?;

    db::users::update_spotify_tokens(
        &state.db,
        user_id,
        &tokens.access_token,
        tokens.refresh_token.as_deref(),
        tokens.expires_at,
    )
    .await?;

    Ok(Redirect::to("http://127.0.0.1:5173?spotify_linked=true").into_response())
}

pub async fn spotify_status(
    AuthSession { user }: AuthSession,
) -> Json<SpotifyStatusResponse> {
    Json(SpotifyStatusResponse {
        linked: user.spotify_refresh_token.is_some(),
    })
}

pub async fn spotify_unlink(
    State(state): State<AppState>,
    AuthSession { user }: AuthSession,
) -> Result<StatusCode, AppError> {
    db::users::clear_spotify_tokens(&state.db, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
