use axum::{
    Router,
    routing::get,
    extract::{Query, State},
    response::{Json, Redirect},
};
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tower_http::cors::{CorsLayer, Any};

mod auth;
mod error;
mod types;
mod spotify;
mod ticketmaster;

use error::AppError;

struct AppState {
    csrf_token: Option<String>,
    access_token: Option<String>,
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct ConcertsParams {
    country: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let shared_state = Arc::new(Mutex::new(AppState {
        csrf_token: None,
        access_token: None,
    }));

    let app = Router::new()
        .route("/auth/login", get(login_handler))
        .route("/auth/callback", get(callback_handler))
        .route("/concerts", get(concerts_handler))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

async fn login_handler(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Redirect, AppError> {
    let (auth_url, csrf_token) = auth::generate_auth_url()
        .map_err(AppError::Internal)?;

    state.lock().unwrap().csrf_token = Some(csrf_token);

    Ok(Redirect::to(&auth_url.to_string()))
}

async fn callback_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, AppError> {
    let stored_csrf = state
        .lock()
        .unwrap()
        .csrf_token
        .clone()
        .ok_or(AppError::Internal("CSRF token missing".to_string()))?;

    if params.state != stored_csrf {
        return Err(AppError::Unauthorized("CSRF token mismatch".to_string()));
    }

    let access_token = auth::exchange_code(params.code).await
        .map_err(AppError::Internal)?;

    state.lock().unwrap().access_token = Some(access_token);

    Ok(Redirect::to("http://127.0.0.1:5173?loggedin=true"))
}

async fn concerts_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<ConcertsParams>,
) -> Result<Json<Vec<types::Concert>>, AppError> {
    let access_token = state
        .lock()
        .unwrap()
        .access_token
        .clone()
        .ok_or(AppError::Unauthorized("User not authenticated".to_string()))?;

    let artists = spotify::get_top_artists(&access_token).await
        .map_err(AppError::Internal)?;
    let concerts = ticketmaster::get_concerts(&artists, &params.country).await
        .map_err(AppError::Internal)?;

    Ok(Json(concerts))
}