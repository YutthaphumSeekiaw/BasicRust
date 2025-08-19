use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::service::ServiceError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Database connection error")]
    DatabaseConnection,
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Service(ServiceError::OrderNotFound { id }) => {
                (StatusCode::NOT_FOUND, format!("Order with id {} not found", id))
            }
            ApiError::Service(ServiceError::Validation(msg)) => {
                (StatusCode::BAD_REQUEST, format!("Validation error: {}", msg))
            }
            ApiError::Service(ServiceError::Repository(_)) => {
                tracing::error!("Repository error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            ApiError::Service(ServiceError::StatusReporting(_)) => {
                tracing::warn!("Status reporting error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            ApiError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, format!("Validation error: {}", msg))
            }
            ApiError::DatabaseConnection => {
                tracing::error!("Database connection error");
                (StatusCode::SERVICE_UNAVAILABLE, "Service temporarily unavailable".to_string())
            }
            ApiError::Internal => {
                tracing::error!("Internal server error: {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now()
        }));

        (status, body).into_response()
    }
}