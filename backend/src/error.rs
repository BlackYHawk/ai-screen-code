use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rusqlite::Error as SqliteError;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Image processing error: {0}")]
    ImageError(String),

    #[error("AI service error: {0}")]
    AiServiceError(String),

    #[error("Invalid model: {0}")]
    InvalidModel(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<SqliteError> for AppError {
    fn from(err: SqliteError) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ImageError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::AiServiceError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::InvalidModel(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
