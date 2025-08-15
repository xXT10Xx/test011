use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Forbidden")]
    Forbidden,
    
    #[error("Not found")]
    NotFound,
    
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Validation(ref e) => {
                tracing::warn!("Validation error: {:?}", e);
                (StatusCode::BAD_REQUEST, "Validation error")
            }
            AppError::Jwt(ref e) => {
                tracing::warn!("JWT error: {:?}", e);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AppError::Bcrypt(ref e) => {
                tracing::error!("Bcrypt error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Config(ref e) => {
                tracing::error!("Configuration error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "details": match self {
                AppError::Validation(e) => Some(format!("{}", e)),
                _ => None,
            }
        }));

        (status, body).into_response()
    }
}