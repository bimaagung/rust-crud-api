use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::domain::errors::DomainError;

// Error wrapper for HTTP responses — Go: type AppError struct { Code int; Message string }
// Implements IntoResponse so Axum can automatically convert it to an HTTP response.
pub struct HttpError(pub DomainError);

// Go: implicit via error interface — here we map DomainError into HttpError automatically
impl From<DomainError> for HttpError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

// Go equivalent: func (e *AppError) Render(c *gin.Context) { c.JSON(e.Code, gin.H{"error": e.Message}) }
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        // Map domain error variants to HTTP status codes
        let (status, message) = match &self.0 {
            DomainError::NotFound(msg)   => (StatusCode::NOT_FOUND, format!("Not Found: {}", msg)),
            DomainError::Validation(msg) => (StatusCode::BAD_REQUEST, format!("Validation Error: {}", msg)),
            DomainError::Database(msg)   => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database Error: {}", msg)),
            DomainError::Internal(msg)   => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: {}", msg)),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
