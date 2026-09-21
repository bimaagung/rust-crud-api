use serde::Deserialize;

use crate::application::post::dtos::{CreatePostDto, UpdatePostDto};

// HTTP request struct for JSON binding — Go: type CreatePostRequest struct { Title, Body string `json:"..." binding:"required"` }
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

// Mapper from HTTP request → application DTO — Go: param := usecase.CreatePostParam{Title: req.Title, ...}
impl From<CreatePostRequest> for CreatePostDto {
    fn from(req: CreatePostRequest) -> Self {
        Self { title: req.title, body: req.body }
    }
}

// HTTP request struct for partial update — Option<T> = nullable field, Go: *string / *bool
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

// Mapper from HTTP request → application DTO
impl From<UpdatePostRequest> for UpdatePostDto {
    fn from(req: UpdatePostRequest) -> Self {
        Self { title: req.title, body: req.body, published: req.published }
    }
}
