use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub enum AppError {
    Unauthorized(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        eprintln!("Error {}: {}", status, message);
        (status, message).into_response()
    }
}
