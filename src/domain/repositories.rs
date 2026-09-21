use crate::domain::entities::Post;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Post>, DomainError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError>;
    async fn create(&self, title: String, body: String) -> Result<Post, DomainError>;
    async fn update(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
        published: Option<bool>,
    ) -> Result<Post, DomainError>;
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}
