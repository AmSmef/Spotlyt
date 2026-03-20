use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, AuthUrl, TokenUrl
};
use oauth2::basic::BasicClient;
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use url::Url;

const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub async fn authenticate() -> Result<String, String> {

    // read id, secret, and redirect uri from .env
    let client_id = std::env::var("S_ID")
        .map_err(|_| "Missing SPOTIFY_CLIENT_ID".to_string())?;
    let client_secret = std::env::var("S_SECRET")
        .map_err(|_| "Missing SPOTIFY_CLIENT_SECRET".to_string())?;
    let redirect_uri = std::env::var("S_REDIRECT_URI")
        .map_err(|_| "Missing SPOTIFY_REDIRECT_URI".to_string())?;

    // oauth client
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(AUTH_URL.to_string()).unwrap(),
        Some(TokenUrl::new(TOKEN_URL.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    // generate spotify login url
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-top-read".to_string()))
        .url();

    // opening web browser
    println!("Opening Spotify login in browser...");
    open::that(auth_url.to_string()).map_err(|e| e.to_string())?;

    // spin up local server to catch the callback
    let listener = TcpListener::bind("127.0.0.1:8000")
        .map_err(|e| format!("Failed to bind port 8000: {e}"))?;

    let (mut stream, _) = listener.accept()
        .map_err(|e| format!("Failed to accept connection: {e}"))?;

    // reading callback
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)
        .map_err(|e| format!("Failed to read request: {e}"))?;

    // parse callback url to extract the auth code
    let redirect_path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or("Could not parse redirect URL")?;

    let url = Url::parse(&format!("http://localhost{redirect_path}"))
        .map_err(|e| format!("Failed to parse URL: {e}"))?;

    let code = url
        .query_pairs()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.into_owned())
        .ok_or("No auth code in callback")?;

    let state = url
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.into_owned())
        .ok_or("No state in callback")?;

    // csrf check
    if state != *csrf_token.secret() {
        return Err("CSRF token mismatch".to_string());
    }

    // send a response to the browser
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Login successful!</h1><p>You can close this tab and return to Spotlyt.</p></body></html>";
    stream.write_all(response.as_bytes())
        .map_err(|e| format!("Failed to send response: {e}"))?;

    // exchange auth code for token
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| format!("Token exchange failed: {e}"))?;

    Ok(token_result.access_token().secret().clone())
}