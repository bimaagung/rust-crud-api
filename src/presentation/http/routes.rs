use axum::Router;
use axum::routing::get;

use crate::presentation::http::handlers::{
    create_post, delete_post, get_post_by_id, get_posts, update_post,
};
use crate::presentation::http::state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/posts", get(get_posts).post(create_post))
        .route(
            "/api/posts/{id}",
            get(get_post_by_id).put(update_post).delete(delete_post),
        )
        .with_state(state)
}
