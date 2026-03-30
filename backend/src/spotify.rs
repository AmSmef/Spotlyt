use chrono::Utc;
use reqwest::Client;
use uuid::Uuid;
use crate::{db, types::Artist};

pub async fn get_top_artists(access_token: &str) -> Result<Vec<Artist>, String> {
    let client = Client::new();

    // medium_term gives data from the past 6 months
    // could also use short_term (4 weeks) or long_term (all time)
    let response = client
        .get("https://api.spotify.com/v1/me/top/artists?limit=50&time_range=long_term")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    let artists = json["items"]
        .as_array()
        .ok_or_else(|| format!("No items in Spotify response: {}", json))?
        .iter()
        .map(|item| Artist {
            id: item["id"].as_str().unwrap_or("").to_string(),
            name: item["name"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(artists)
}

/// Returns a valid access token for the user, refreshing it from Spotify if it
/// is within 60 seconds of expiry. Writes the new token back to the DB on refresh.
pub async fn get_valid_access_token(
    pool: &db::DbPool,
    user_id: Uuid,
    current_access_token: &str,
    refresh_token: &str,
    expires_at: chrono::DateTime<Utc>,
) -> Result<String, String> {
    if expires_at - Utc::now() > chrono::Duration::seconds(60) {
        return Ok(current_access_token.to_string());
    }
    refresh_access_token(pool, user_id, refresh_token).await
}

async fn refresh_access_token(
    pool: &db::DbPool,
    user_id: Uuid,
    refresh_token: &str,
) -> Result<String, String> {
    let client_id = std::env::var("S_ID")
        .map_err(|_| "Missing Spotify client ID".to_string())?;
    let client_secret = std::env::var("S_SECRET")
        .map_err(|_| "Missing Spotify client secret".to_string())?;

    let client = Client::new();
    let response: serde_json::Value = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ])
        .send()
        .await
        .map_err(|e| format!("Token refresh request failed: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse refresh response: {e}"))?;

    let new_access_token = response["access_token"]
        .as_str()
        .ok_or("No access_token in refresh response")?
        .to_string();

    let expires_in = response["expires_in"].as_u64().unwrap_or(3600);
    let new_expires_at = Utc::now() + chrono::Duration::seconds(expires_in as i64);

    // Spotify may or may not return a new refresh_token; only update if present
    let new_refresh_token = response["refresh_token"].as_str();

    db::users::update_spotify_tokens(pool, user_id, &new_access_token, new_refresh_token, new_expires_at)
        .await
        .map_err(|e| format!("Failed to persist refreshed tokens: {e}"))?;

    Ok(new_access_token)
}
