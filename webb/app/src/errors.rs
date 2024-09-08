use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

// APP ERRORS

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    Conflict(String),
    InternalServerError,
    BodyParsingError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, String::from("Unauthorized")),
            Self::Conflict(message) => (
                StatusCode::CONFLICT,
                format!("Resource already exists: {}", message),
            ),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", message),
            ),
        };
        (status, Json(json!({ "status": err_msg }))).into_response()
    }
}

// DATABASE ERRORS

#[derive(Debug)]
pub enum DBError {
    Conflict(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Conflict(message) => write!(f, "{}", message),
            Self::NotFound(message) => write!(f, "{}", message),
            Self::InternalError(message) => write!(f, "{}", message),
        }
    }
}
