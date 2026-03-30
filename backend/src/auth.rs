use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, AuthUrl, TokenUrl,
};
use oauth2::basic::BasicClient;
use url::Url;

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

fn build_spotify_oauth_client() -> Result<BasicClient, String> {
    let client_id = std::env::var("S_ID")
        .map_err(|_| "Missing Spotify client ID".to_string())?;
    let client_secret = std::env::var("S_SECRET")
        .map_err(|_| "Missing Spotify client secret".to_string())?;
    let redirect_uri = std::env::var("S_REDIRECT_URI")
        .map_err(|_| "Missing Spotify redirect URI".to_string())?;

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(SPOTIFY_AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(SPOTIFY_TOKEN_URL.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap()))
}

pub fn generate_spotify_auth_url() -> Result<(String, String), String> {
    let client = build_spotify_oauth_client()?;
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-top-read".to_string()))
        .url();
    Ok((auth_url.to_string(), csrf_token.secret().clone()))
}

pub struct SpotifyTokens {
    pub access_token: String,
    /// Only present on initial exchange; not always re-issued on refresh.
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub async fn exchange_spotify_code(code: String) -> Result<SpotifyTokens, String> {
    let client = build_spotify_oauth_client()?;
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| format!("Token exchange failed: {e}"))?;

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result.refresh_token().map(|t| t.secret().clone());
    let expires_in = token_result
        .expires_in()
        .unwrap_or(std::time::Duration::from_secs(3600));
    let expires_at = chrono::Utc::now()
        + chrono::Duration::from_std(expires_in).unwrap_or(chrono::Duration::hours(1));

    Ok(SpotifyTokens { access_token, refresh_token, expires_at })
}

// --- Google OAuth ---

pub fn generate_google_auth_url() -> Result<(String, String), String> {
    let client_id = std::env::var("G_ID")
        .map_err(|_| "Missing Google client ID".to_string())?;
    let redirect_uri = std::env::var("G_REDIRECT_URI")
        .map_err(|_| "Missing Google redirect URI".to_string())?;

    // Use a UUID as the CSRF token (no oauth2 crate needed for Google flow)
    let csrf_token = uuid::Uuid::new_v4().to_string();

    let mut auth_url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "openid email profile")
        .append_pair("state", &csrf_token)
        .append_pair("access_type", "offline");

    Ok((auth_url.to_string(), csrf_token))
}

pub struct GoogleUser {
    pub google_id: String,
    pub email: String,
    pub display_name: String,
}

pub async fn exchange_google_code(code: &str) -> Result<GoogleUser, String> {
    let client_id = std::env::var("G_ID")
        .map_err(|_| "Missing Google client ID".to_string())?;
    let client_secret = std::env::var("G_SECRET")
        .map_err(|_| "Missing Google client secret".to_string())?;
    let redirect_uri = std::env::var("G_REDIRECT_URI")
        .map_err(|_| "Missing Google redirect URI".to_string())?;

    let client = reqwest::Client::new();

    let token_resp: serde_json::Value = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| format!("Google token exchange failed: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse Google token response: {e}"))?;

    let access_token = token_resp["access_token"]
        .as_str()
        .ok_or("No access_token in Google response")?
        .to_string();

    let userinfo: serde_json::Value = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| format!("Google userinfo request failed: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse Google userinfo: {e}"))?;

    Ok(GoogleUser {
        google_id: userinfo["sub"].as_str().ok_or("No sub in Google userinfo")?.to_string(),
        email: userinfo["email"].as_str().unwrap_or("").to_string(),
        display_name: userinfo["name"].as_str().unwrap_or("").to_string(),
    })
}
