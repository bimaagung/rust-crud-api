use actix_web::{HttpResponse, web};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{NewPost, Post, UpdatePost};
use crate::schema::posts;

// CREATE: POST /posts
pub async fn create_post(
    pool: web::Data<DbPool>,
    item: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new_post = item.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get()?;

        diesel::insert_into(posts::table)
            .values(&new_post)
            .returning(Post::as_select())
            .get_result(&mut conn)
            .map_err(ApiError::from)
    })
    .await
    .map_err(|_| ApiError::InternalServerError)??;

    Ok(HttpResponse::Created().json(post))
}

// READ ALL: GET /posts
pub async fn get_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, ApiError> {
    let list = web::block(move || {
        let mut conn = pool.get()?;
        posts::table
            .select(Post::as_select())
            .load(&mut conn)
            .map_err(ApiError::from)
    })
    .await
    .map_err(|_| ApiError::InternalServerError)??;

    Ok(HttpResponse::Ok().json(list))
}

// READ BY ID: GET /posts/{id}
pub async fn get_post_by_id(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = post_id.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get()?;
        posts::table
            .find(id)
            .select(Post::as_select())
            .first(&mut conn)
            .map_err(ApiError::from)
    })
    .await
    .map_err(|_| ApiError::InternalServerError)??;

    Ok(HttpResponse::Ok().json(post))
}

pub async fn update_post(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
    item: web::Json<UpdatePost>,
) -> Result<HttpResponse, ApiError> {
    let id = post_id.into_inner();
    let update_data = item.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get()?;
        diesel::update(posts::table.find(id))
            .set(&update_data)
            .returning(Post::as_select())
            .get_result(&mut conn)
            .map_err(ApiError::from)
    })
    .await
    .map_err(|_| ApiError::InternalServerError)??;

    Ok(HttpResponse::Ok().json(post))
}

// DELETE: DELETE /posts/{id}
pub async fn delete_post(
    pool: web::Data<DbPool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = post_id.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        let num_deleted = diesel::delete(posts::table.find(id))
            .execute(&mut conn)
            .map_err(ApiError::from)?;

        if num_deleted == 0 {
            return Err(ApiError::NotFound(format!("Post with id {} not found", id)));
        }
        Ok(())
    })
    .await
    .map_err(|_| ApiError::InternalServerError)??;

    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(get_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post_by_id))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post)),
    );
}
