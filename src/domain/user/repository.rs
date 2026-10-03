use async_trait::async_trait;

use crate::domain::errors::DomainError;
use crate::domain::user::entity::{Role, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        name: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, DomainError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError>;
}
