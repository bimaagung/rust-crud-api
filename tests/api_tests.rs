use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

use rust_crud::application::post::PostUseCase;
use rust_crud::domain::errors::DomainError;
use rust_crud::domain::post::{Post, PostRepository};
use rust_crud::presentation::http::{AppState, create_routes};

#[derive(Default)]
struct MockRepo {
    posts: Mutex<Vec<Post>>,
}

#[async_trait::async_trait]
impl PostRepository for MockRepo {
    async fn find_all(&self) -> Result<Vec<Post>, DomainError> {
        Ok(self.posts.lock().unwrap().clone())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DomainError> {
        Ok(self
            .posts
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned())
    }

    async fn create(&self, title: String, body: String) -> Result<Post, DomainError> {
        let mut posts = self.posts.lock().unwrap();
        let post = Post {
            id: (posts.len() as i32) + 1,
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
        let len = posts.len();
        posts.retain(|p| p.id != id);
        if posts.len() == len {
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
async fn test_api_crud_flow() {
    let repo = Arc::new(MockRepo::default());
    let use_case = Arc::new(PostUseCase::new(repo));
    let app = create_routes(AppState {
        post_use_case: use_case,
    });

    // 1. GET /api/posts (initially empty)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/posts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. POST /api/posts
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/posts")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"title":"Hello Axum","body":"Content body"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let post: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(post["title"], "Hello Axum");
    assert_eq!(post["body"], "Content body");
    assert_eq!(post["published"], false);
    assert_eq!(post["id"], 1);

    // 3. GET /api/posts/1
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/posts/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 4. PUT /api/posts/1
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/posts/1")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"title":"Updated Title","published":true}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let updated: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(updated["title"], "Updated Title");
    assert_eq!(updated["published"], true);

    // 5. DELETE /api/posts/1
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/posts/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 6. GET /api/posts/1 -> 404 Not Found
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/posts/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let error_resp: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(error_resp["error"].as_str().unwrap().contains("Not Found"));
}
