use axum::response::IntoResponse;
use thiserror::Error;

use crate::common::response::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::DatabaseError(inner) => {
                tracing::error!("Database error: {:?}", inner);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::UserNotFound => (axum::http::StatusCode::NOT_FOUND, "User not found".to_string()),
            AppError::ValidationError(msg) => {
                tracing::warn!("Validation error: {}", msg);
                (axum::http::StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::Conflict(msg) => {
                tracing::warn!("Conflict: {}", msg);
                (axum::http::StatusCode::CONFLICT, msg.clone())
            }
            other => {
                tracing::error!("Unhandled error: {:?}", other);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        let body = axum::Json(ApiResponse::<()>::error(error_message));
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

