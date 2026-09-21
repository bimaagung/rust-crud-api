use crate::application::dtos::{CreatePostDto, UpdatePostDto};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
}

impl From<CreatePostRequest> for CreatePostDto {
    fn from(req: CreatePostRequest) -> Self {
        Self {
            title: req.title,
            body: req.body,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

impl From<UpdatePostRequest> for UpdatePostDto {
    fn from(req: UpdatePostRequest) -> Self {
        Self {
            title: req.title,
            body: req.body,
            published: req.published,
        }
    }
}
