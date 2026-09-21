use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation Error: {0}")]
    Validation(String),

    #[error("Database Error: {0}")]
    Database(String),

    #[error("Internal Server Error")]
    Internal(String),
}
