use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::domain::entities::Post;
use crate::presentation::http::dtos::{CreatePostRequest, UpdatePostRequest};
use crate::presentation::http::errors::HttpError;
use crate::presentation::http::state::AppState;

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), HttpError> {
    let post = state.post_use_case.create_post(payload.into()).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, HttpError> {
    let posts = state.post_use_case.get_all_posts().await?;
    Ok(Json(posts))
}

pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, HttpError> {
    let post = state.post_use_case.get_post_by_id(id).await?;
    Ok(Json(post))
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Post>, HttpError> {
    let post = state.post_use_case.update_post(id, payload.into()).await?;
    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, HttpError> {
    state.post_use_case.delete_post(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
