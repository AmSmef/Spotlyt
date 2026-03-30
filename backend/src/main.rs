use axum::{
    extract::FromRef,
    http::{header, HeaderValue, Method},
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::CorsLayer;

mod auth;
mod cookies;
mod db;
mod error;
mod handlers;
mod spotify;
mod ticketmaster;
mod types;

use error::AppError;
use types::User;

#[derive(Clone)]
pub struct AppState {
    pub db: db::DbPool,
}

// Allows sub-types of AppState to be extracted in custom extractors
impl FromRef<AppState> for db::DbPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

/// Custom extractor that validates the session cookie and returns the authenticated user.
pub struct AuthSession {
    pub user: User,
}

impl<S> axum::extract::FromRequestParts<S> for AuthSession
where
    AppState: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = cookies::extract_session_cookie(&parts.headers)
            .ok_or_else(|| AppError::Unauthorized("Not authenticated".to_string()))?;

        let user = db::sessions::get_session_user(&app_state.db, &token)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::Unauthorized("Invalid or expired session".to_string()))?;

        Ok(AuthSession { user })
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = db::connect(&database_url).await;

    sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await
        .expect("migrations/ directory not found — run from backend/")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(
            std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:5173".to_string())
                .parse::<HeaderValue>()
                .expect("Invalid FRONTEND_URL"),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        // Auth: password
        .route("/auth/register", post(handlers::auth_password::register))
        .route("/auth/login", post(handlers::auth_password::login))
        .route("/auth/logout", post(handlers::auth_password::logout))
        // Auth: Google OAuth
        .route("/auth/google", get(handlers::auth_google::google_login))
        .route("/auth/google/callback", get(handlers::auth_google::google_callback))
        // Auth: Spotify account linking (requires being logged in)
        .route("/auth/spotify/link", get(handlers::auth_spotify::spotify_link))
        .route("/auth/spotify/callback", get(handlers::auth_spotify::spotify_callback))
        .route("/auth/spotify/status", get(handlers::auth_spotify::spotify_status))
        .route("/auth/spotify/unlink", delete(handlers::auth_spotify::spotify_unlink))
        // User profile
        .route("/users/me", get(handlers::users::me))
        // Concerts
        .route("/concerts", get(handlers::concerts::concerts_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
