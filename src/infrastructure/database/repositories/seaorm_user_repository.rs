use async_trait::async_trait;
use sea_orm::*;

use crate::domain::errors::DomainError;
use crate::domain::user::{Role, User, UserRepository};
use crate::infrastructure::database::entities::user::{self, Entity as UserEntity};

#[derive(Clone)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, DomainError> {
        let model = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(model.map(Into::into))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, DomainError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(model.map(Into::into))
    }

    async fn create(
        &self,
        name: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<User, DomainError> {
        let new_model = user::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(password),
            role: Set(role.to_string()),
            ..Default::default()
        };

        let inserted = new_model
            .insert(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(inserted.into())
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError> {
        let count = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .count(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(count > 0)
    }
}
