use thiserror::Error;

// Typed domain errors — Go equivalent: var ErrNotFound = errors.New("...") / custom error types
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("Not Found: {0}")]
    NotFound(String), // Go: return nil, ErrNotFound

    #[error("Validation Error: {0}")]
    Validation(String), // Go: return nil, ErrValidation

    #[error("Database Error: {0}")]
    Database(String), // Go: return nil, fmt.Errorf("db error: %w", err)

    #[error("Internal Server Error")]
    Internal(String), // Go: return nil, ErrInternal
}
