use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::domain::errors::DomainError;

pub struct HttpError(pub DomainError);

impl From<DomainError> for HttpError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, format!("Not Found: {}", msg)),
            DomainError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Validation Error: {}", msg),
            ),
            DomainError::Database(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database Error: {}", msg),
            ),
            DomainError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
