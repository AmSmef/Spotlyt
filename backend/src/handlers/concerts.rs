use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use crate::{error::AppError, spotify, ticketmaster, types::Concert, AppState, AuthSession};

#[derive(Deserialize)]
pub struct ConcertsParams {
    pub country: String,
}

pub async fn concerts_handler(
    State(state): State<AppState>,
    AuthSession { user }: AuthSession,
    Query(params): Query<ConcertsParams>,
) -> Result<Json<Vec<Concert>>, AppError> {
    let refresh_token = user
        .spotify_refresh_token
        .ok_or_else(|| AppError::Unauthorized("Spotify account not linked".to_string()))?;

    let current_access_token = user.spotify_access_token.unwrap_or_default();
    let expires_at = user
        .spotify_token_expires_at
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::hours(1));

    let access_token = spotify::get_valid_access_token(
        &state.db,
        user.id,
        &current_access_token,
        &refresh_token,
        expires_at,
    )
    .await
    .map_err(AppError::Internal)?;

    let artists = spotify::get_top_artists(&access_token)
        .await
        .map_err(AppError::Internal)?;

    let concerts = ticketmaster::get_concerts(&artists, &params.country)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(concerts))
}
