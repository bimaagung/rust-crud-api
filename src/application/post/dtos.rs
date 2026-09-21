use serde::{Deserialize, Serialize};

// Internal DTO for use case input — Go equivalent: type CreatePostParam struct { Title, Body string }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostDto {
    pub title: String,
    pub body: String,
}

// Partial update params — Go equivalent: type UpdatePostParam struct { Title, Body *string; Published *bool }
// Option<T> = *T in Go: Some("val") means provided, None means nil (not provided)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}
