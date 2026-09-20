use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenvy::dotenv;
use std::env;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    log::info!("Server running at http://127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api").configure(handlers::init_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
