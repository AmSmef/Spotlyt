use reqwest::Client;
use crate::types::Artist;

pub async fn get_top_artists(access_token: &str) -> Result<Vec<Artist>, String> {
    let client = Client::new();

    let response = client
    .get("https://api.spotify.com/v1/me/top/artists?limit=20&time_range=medium_term")
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
    .ok_or("No items in response")?
    .iter()
    .map(|item| Artist {
        id: item["id"].as_str().unwrap_or("").to_string(),
        name: item["name"].as_str().unwrap_or("").to_string(),
    })
    .collect();

    Ok(artists)

}