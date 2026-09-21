use async_trait::async_trait;
use sea_orm::*;

use crate::domain::entities::Post;
use crate::domain::errors::DomainError;
use crate::domain::repositories::PostRepository;
use crate::infrastructure::database::entities::post::{self, Entity as PostEntity};

#[derive(Clone)]
pub struct SeaOrmPostRepository {
    db: DatabaseConnection,
}

impl SeaOrmPostRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostRepository for SeaOrmPostRepository {
    async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
        let models = PostEntity::find()
            .all(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError> {
        let model = PostEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(model.map(Into::into))
    }

    async fn create(&self, title: String, body: String) -> Result<Post, DomainError> {
        let new_model = post::ActiveModel {
            title: Set(title),
            body: Set(body),
            published: Set(false),
            ..Default::default()
        };

        let inserted = new_model
            .insert(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(inserted.into())
    }

    async fn update(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
        published: Option<bool>,
    ) -> Result<Post, DomainError> {
        let existing: post::Model = PostEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Data not found".into()))?;

        let mut active: post::ActiveModel = existing.into();

        if let Some(t) = title {
            active.title = Set(t);
        }
        if let Some(b) = body {
            active.body = Set(b);
        }
        if let Some(p) = published {
            active.published = Set(p);
        }

        let updated = active
            .update(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(updated.into())
    }

    async fn delete(&self, id: i32) -> Result<(), DomainError> {
        let res = PostEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        if res.rows_affected == 0 {
            return Err(DomainError::NotFound(format!(
                "Post with id {} not found",
                id
            )));
        }

        Ok(())
    }
}
