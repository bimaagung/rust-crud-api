use axum::Router;

use crate::presentation::http::post::post_routes;
use crate::presentation::http::state::AppState;

// Root router — Go: func NewRouter(h *Handler) *gin.Engine { r := gin.Default(); r.Group("/api") ... }
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api/posts", post_routes()) // mount post sub-routes under /api/posts
        .with_state(state)                 // Go: pass handler struct; Axum: inject shared state
}
