use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostDto {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}
