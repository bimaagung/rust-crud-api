use async_trait::async_trait;

use crate::domain::errors::DomainError;
use crate::domain::post::entity::Post;

// Contract for data access — Go equivalent: type PostRepository interface { ... }
// The infrastructure layer implements this trait; the domain never knows about the DB.
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Post>, DomainError>;                                          // Go: FindAll(ctx) ([]Post, error)
    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError>;                           // Go: FindByID(ctx, id) (*Post, error)
    async fn create(&self, title: String, body: String) -> Result<Post, DomainError>;                   // Go: Create(ctx, title, body) (Post, error)
    async fn update(
        &self,
        id: i32,
        title: Option<String>,   // Go: *string  — None means "not provided by caller"
        body: Option<String>,    // Go: *string
        published: Option<bool>, // Go: *bool
    ) -> Result<Post, DomainError>;
    async fn delete(&self, id: i32) -> Result<(), DomainError>;                                         // Go: Delete(ctx, id) error
}
