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
mod types;
mod spotify;
mod ticketmaster;

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

    let shared_state = Arc::nw(Mutex::new(AppState {
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
) -> Result<Redirect, String> {
    let (auth_url, csrf_token) = auth::generate_auth_url()?;

    state.lock().unwrap().csrf_token = Some(csrf_token);

    Ok(Redirect::to(&auth_url.to_string()))
}

async fn callback_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<CallbackParams>,
) -> Result<Json<serde_json::Value>, String> {
    let stored_csrf = state
        .lock()
        .unwrap()
        .csrf_token
        .clone()
        .ok_or("CSRF token missing".to_string())?;

    if params.state != stored_csrf {
        return Err("CSRF token mismatch".to_string());
    }

    let access_token = auth::exchange_code(params.code).await?;

    state.lock().unwrap().access_token = Some(access_token);

    Ok(Json(serde_json::json!({ "message": "Login successful" })))
}

async fn concerts_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<ConcertsParams>,
) -> Result<Json<Vec<types::Concert>>, String> {
    let access_token = state
        .lock()
        .unwrap()
        .access_token
        .clone()
        .ok_or("User not authenticated".to_string())?;

    let artists = spotify::get_top_artists(&access_token).await?;
    let concerts = ticketmaster::get_concerts(&artists, &params.country).await?;

    Ok(Json(concerts))
}