use axum::Router;
use axum::routing::get;

use crate::presentation::http::post::handlers::{
    create_post, delete_post, get_post_by_id, get_posts, update_post,
};
use crate::presentation::http::state::AppState;

// Route registration — Go: r.GET("/api/posts", handler.GetPosts); r.POST("/api/posts", handler.CreatePost)
pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_posts).post(create_post))           // GET + POST /api/posts
        .route("/{id}", get(get_post_by_id).put(update_post).delete(delete_post)) // GET + PUT + DELETE /api/posts/:id
}
