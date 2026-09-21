use std::sync::Arc;

use crate::application::post::PostUseCase;

// Shared app state injected into every handler — Go equivalent:
//   type Handler struct { postUseCase *usecase.PostUseCase }
// In Axum, State<T> replaces the handler struct pattern from Gin/Fiber.
#[derive(Clone)]
pub struct AppState {
    pub post_use_case: Arc<PostUseCase>, // Arc = shared ownership across async threads (Go: no equivalent needed, GC handles it)
}
