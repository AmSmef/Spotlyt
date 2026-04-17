use serde::{Deserialize, Serialize};
use unidecode::unidecode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Concert {
    pub artist_names: Vec<String>,
    pub event_name: String,
    pub venue: String,
    pub city: String,
    pub date: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub display_name: String,
    pub google_id: Option<String>,
    pub spotify_refresh_token: Option<String>,
    pub spotify_access_token: Option<String>,
    pub spotify_token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Normalises every artist name to be lowercase, without accents and without punctuation.
// Used when comparing artist names between Spotify and Ticketmaster.
pub fn normalise_artist_name(name: &str) -> String {
    unidecode(name)
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
