use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, AuthUrl, TokenUrl
};
use oauth2::basic::BasicClient;

const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

fn build_oauth_client() -> Result<BasicClient, String> {

    // read id, secret, and redirect uri from .env
    let client_id = std::env::var("S_ID")
        .map_err(|_| "Missing Spotify client Id".to_string())?;
    let client_secret = std::env::var("S_SECRET")
        .map_err(|_| "Missing Spotify client secret".to_string())?;
    let redirect_uri = std::env::var("S_REDIRECT_URI")
        .map_err(|_| "Missing Spotify redirect URI".to_string())?;

    // oauth client
    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap()))
}

pub fn generate_auth_url() -> Result<(String, String), String> {

    let client = build_oauth_client()?;

    // generate spotify login url
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-top-read".to_string()))
        .url();

    Ok((auth_url.to_string(), csrf_token.secret().clone()))
}

pub async fn exchange_code(code: String) -> Result<String, String> {

    let client = build_oauth_client()?;

    // exchange auth code for token
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| format!("Token exchange failed: {e}"))?;

    Ok(token_result.access_token().secret().clone())
}