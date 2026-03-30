use axum::http::HeaderMap;

/// Reads the `session` cookie value from request headers.
pub fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .map(|s| s.trim())
                .find(|s| s.starts_with("session="))
                .and_then(|s| s.strip_prefix("session="))
                .map(String::from)
        })
}

/// Reads an arbitrary named cookie value from request headers.
pub fn extract_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    let prefix = format!("{}=", name);
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .map(|s| s.trim())
                .find(|s| s.starts_with(&prefix))
                .and_then(|s| s.strip_prefix(&prefix))
                .map(String::from)
        })
}

/// Builds a `Set-Cookie` header value for a session token (30-day expiry).
pub fn session_cookie(token: &str) -> String {
    format!(
        "session={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=2592000",
        token
    )
}

/// Builds a `Set-Cookie` header value that clears the session cookie.
pub fn clear_session_cookie() -> &'static str {
    "session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"
}
