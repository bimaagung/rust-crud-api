use async_trait::async_trait;
use sea_orm::*;

use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostRepository};
use crate::infrastructure::database::entities::post::{self, Entity as PostEntity};

// Infrastructure implementation of PostRepository — Go: type SeaOrmPostRepository struct { db *gorm.DB }
// All SQL queries live here. Domain and application layers never import this struct directly.
#[derive(Clone)]
pub struct SeaOrmPostRepository {
    db: DatabaseConnection, // Go equivalent: *sql.DB or *gorm.DB (connection pool managed internally)
}

impl SeaOrmPostRepository {
    // Go: func NewSeaOrmPostRepository(db *gorm.DB) *SeaOrmPostRepository
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// Explicit interface implementation — Go: implicit (just match method signatures)
#[async_trait]
impl PostRepository for SeaOrmPostRepository {
    // SELECT * FROM posts — Go: r.db.Find(&posts).Error
    async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
        let models = PostEntity::find()
            .all(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?; // Go: if err != nil { return nil, err }

        Ok(models.into_iter().map(Into::into).collect()) // map DB model → domain Post
    }

    // SELECT * FROM posts WHERE id = ? — Go: r.db.First(&post, id).Error
    // Returns Ok(None) if not found (like returning nil, nil in Go)
    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError> {
        let model = PostEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(model.map(Into::into)) // Some(model) → Some(Post), None → None
    }

    // INSERT INTO posts (title, body, published) VALUES (?, ?, false)
    // Go: r.db.Create(&post).Error
    async fn create(&self, title: String, body: String) -> Result<Post, DomainError> {
        let new_model = post::ActiveModel {
            title: Set(title),        // Set() marks the field to be written — Go: post.Title = title
            body: Set(body),
            published: Set(false),    // default to draft
            ..Default::default()      // unset fields are ignored by SeaORM
        };

        let inserted = new_model
            .insert(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(inserted.into())
    }

    // UPDATE posts SET ... WHERE id = ?
    // Go: r.db.First(&existing, id); if title != nil { existing.Title = *title }; r.db.Save(&existing)
    async fn update(
        &self,
        id: i32,
        title: Option<String>,   // None = not provided (Go: nil pointer)
        body: Option<String>,
        published: Option<bool>,
    ) -> Result<Post, DomainError> {
        // Fetch existing row first — Go: r.db.First(&existing, id)
        let existing: post::Model = PostEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Data not found".into()))?; // Go: if err == ErrRecordNotFound

        // Convert read-only Model to mutable ActiveModel for partial update
        let mut active: post::ActiveModel = existing.into();

        // Apply only the fields that were provided — Go: if title != nil { existing.Title = *title }
        if let Some(t) = title { active.title = Set(t); }
        if let Some(b) = body { active.body = Set(b); }
        if let Some(p) = published { active.published = Set(p); }

        let updated = active
            .update(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        Ok(updated.into())
    }

    // DELETE FROM posts WHERE id = ?
    // Go: result := r.db.Delete(&Post{}, id); if result.RowsAffected == 0 { return ErrNotFound }
    async fn delete(&self, id: i32) -> Result<(), DomainError> {
        let res = PostEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|err| DomainError::Database(err.to_string()))?;

        if res.rows_affected == 0 {
            return Err(DomainError::NotFound(format!("Post with id {} not found", id)));
        }

        Ok(()) // Go: return nil
    }
}
