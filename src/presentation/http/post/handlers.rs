use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::domain::post::entity::Post;
use crate::presentation::http::errors::HttpError;
use crate::presentation::http::post::dtos::{CreatePostRequest, UpdatePostRequest};
use crate::presentation::http::state::AppState;

// POST /api/posts — Go: func (h *PostHandler) CreatePost(c *gin.Context)
// State(state) = dependency injection via Axum; Go equivalent: handler struct with use case field
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>, // Go: c.ShouldBindJSON(&req)
) -> Result<(StatusCode, Json<Post>), HttpError> {
    let post = state.post_use_case.create_post(payload.into()).await?; // payload.into() = mapper to DTO
    Ok((StatusCode::CREATED, Json(post))) // Go: c.JSON(201, post)
}

// GET /api/posts — Go: func (h *PostHandler) GetPosts(c *gin.Context)
pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, HttpError> {
    let posts = state.post_use_case.get_all_posts().await?;
    Ok(Json(posts)) // Go: c.JSON(200, posts)
}

// GET /api/posts/:id — Go: func (h *PostHandler) GetPostByID(c *gin.Context)
pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>, // Go: c.Param("id")
) -> Result<Json<Post>, HttpError> {
    let post = state.post_use_case.get_post_by_id(id).await?;
    Ok(Json(post))
}

// PUT /api/posts/:id — Go: func (h *PostHandler) UpdatePost(c *gin.Context)
pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Post>, HttpError> {
    let post = state.post_use_case.update_post(id, payload.into()).await?;
    Ok(Json(post))
}

// DELETE /api/posts/:id — Go: func (h *PostHandler) DeletePost(c *gin.Context)
pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    state.post_use_case.delete_post(id).await?;
    Ok(StatusCode::NO_CONTENT) // Go: c.Status(204)
}
