use std::sync::Arc;

use crate::application::dtos::{CreatePostDto, UpdatePostDto};
use crate::domain::entities::Post;
use crate::domain::errors::DomainError;
use crate::domain::repositories::PostRepository;

#[derive(Clone)]
pub struct PostUseCase {
    repository: Arc<dyn PostRepository>,
}

impl PostUseCase {
    pub fn new(repository: Arc<dyn PostRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_posts(&self) -> Result<Vec<Post>, DomainError> {
        self.repository.find_all().await
    }

    pub async fn get_post_by_id(&self, id: i32) -> Result<Post, DomainError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Data not found".into()))
    }

    pub async fn create_post(&self, dto: CreatePostDto) -> Result<Post, DomainError> {
        if dto.title.trim().is_empty() {
            return Err(DomainError::Validation("Title cannot be empty".into()));
        }
        if dto.body.trim().is_empty() {
            return Err(DomainError::Validation("Body cannot be empty".into()));
        }

        self.repository.create(dto.title, dto.body).await
    }

    pub async fn update_post(&self, id: i32, dto: UpdatePostDto) -> Result<Post, DomainError> {
        if dto.title.is_none() && dto.body.is_none() && dto.published.is_none() {
            return Err(DomainError::Validation(
                "At least one field must be provided to update".into(),
            ));
        }

        self.repository
            .update(id, dto.title, dto.body, dto.published)
            .await
    }

    pub async fn delete_post(&self, id: i32) -> Result<(), DomainError> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockPostRepository {
        posts: Mutex<Vec<Post>>,
    }

    #[async_trait::async_trait]
    impl PostRepository for MockPostRepository {
        async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
            Ok(self.posts.lock().unwrap().clone())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError> {
            let posts = self.posts.lock().unwrap();
            Ok(posts.iter().find(|p| p.id == id).cloned())
        }

        async fn create(&self, title: String, body: String) -> Result<Post, DomainError> {
            let mut posts = self.posts.lock().unwrap();
            let new_id = (posts.len() as i32) + 1;
            let post = Post {
                id: new_id,
                title,
                body,
                published: false,
            };
            posts.push(post.clone());
            Ok(post)
        }

        async fn update(
            &self,
            id: i32,
            title: Option<String>,
            body: Option<String>,
            published: Option<bool>,
        ) -> Result<Post, DomainError> {
            let mut posts = self.posts.lock().unwrap();
            if let Some(p) = posts.iter_mut().find(|p| p.id == id) {
                if let Some(t) = title {
                    p.title = t;
                }
                if let Some(b) = body {
                    p.body = b;
                }
                if let Some(pub_status) = published {
                    p.published = pub_status;
                }
                Ok(p.clone())
            } else {
                Err(DomainError::NotFound("Data not found".into()))
            }
        }

        async fn delete(&self, id: i32) -> Result<(), DomainError> {
            let mut posts = self.posts.lock().unwrap();
            let initial_len = posts.len();
            posts.retain(|p| p.id != id);
            if posts.len() == initial_len {
                Err(DomainError::NotFound(format!(
                    "Post with id {} not found",
                    id
                )))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_create_post_success() {
        let repo = Arc::new(MockPostRepository::default());
        let use_case = PostUseCase::new(repo);

        let result = use_case
            .create_post(CreatePostDto {
                title: "Test Title".into(),
                body: "Test Body".into(),
            })
            .await;

        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Test Title");
        assert_eq!(post.body, "Test Body");
        assert!(!post.published);
    }

    #[tokio::test]
    async fn test_create_post_validation_error() {
        let repo = Arc::new(MockPostRepository::default());
        let use_case = PostUseCase::new(repo);

        let err = use_case
            .create_post(CreatePostDto {
                title: "   ".into(),
                body: "Test Body".into(),
            })
            .await
            .unwrap_err();

        assert_eq!(err, DomainError::Validation("Title cannot be empty".into()));
    }

    #[tokio::test]
    async fn test_get_all_and_by_id() {
        let repo = Arc::new(MockPostRepository::default());
        let use_case = PostUseCase::new(repo);

        let created = use_case
            .create_post(CreatePostDto {
                title: "Post 1".into(),
                body: "Content 1".into(),
            })
            .await
            .unwrap();

        let all = use_case.get_all_posts().await.unwrap();
        assert_eq!(all.len(), 1);

        let found = use_case.get_post_by_id(created.id).await.unwrap();
        assert_eq!(found.title, "Post 1");

        let not_found = use_case.get_post_by_id(999).await;
        assert!(matches!(not_found, Err(DomainError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_update_post() {
        let repo = Arc::new(MockPostRepository::default());
        let use_case = PostUseCase::new(repo);

        let created = use_case
            .create_post(CreatePostDto {
                title: "Original".into(),
                body: "Original Body".into(),
            })
            .await
            .unwrap();

        let updated = use_case
            .update_post(
                created.id,
                UpdatePostDto {
                    title: Some("Updated".into()),
                    body: None,
                    published: Some(true),
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.title, "Updated");
        assert_eq!(updated.body, "Original Body");
        assert!(updated.published);
    }

    #[tokio::test]
    async fn test_delete_post() {
        let repo = Arc::new(MockPostRepository::default());
        let use_case = PostUseCase::new(repo);

        let created = use_case
            .create_post(CreatePostDto {
                title: "To Delete".into(),
                body: "Content".into(),
            })
            .await
            .unwrap();

        let delete_res = use_case.delete_post(created.id).await;
        assert!(delete_res.is_ok());

        let delete_again = use_case.delete_post(created.id).await;
        assert!(matches!(delete_again, Err(DomainError::NotFound(_))));
    }
}
