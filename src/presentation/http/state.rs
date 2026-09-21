use crate::application::use_cases::PostUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub post_use_case: Arc<PostUseCase>,
}
