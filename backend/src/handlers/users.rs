use axum::response::Json;
use serde::Serialize;
use crate::{error::AppError, AuthSession};

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub username: Option<String>,
    pub display_name: String,
    pub email: Option<String>,
    pub spotify_linked: bool,
}

pub async fn me(AuthSession { user }: AuthSession) -> Result<Json<MeResponse>, AppError> {
    Ok(Json(MeResponse {
        id: user.id.to_string(),
        username: user.username,
        display_name: user.display_name,
        email: user.email,
        spotify_linked: user.spotify_refresh_token.is_some(),
    }))
}
