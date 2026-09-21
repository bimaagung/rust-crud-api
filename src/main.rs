use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use rust_crud::application::use_cases::PostUseCase;
use rust_crud::config::Config;
use rust_crud::domain::repositories::PostRepository;
use rust_crud::infrastructure::database::connection::init_pool;
use rust_crud::infrastructure::database::repositories::SeaOrmPostRepository;
use rust_crud::presentation::http::AppState;
use rust_crud::presentation::http::create_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_crud=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();

    info!("Connecting to database...");
    let db = init_pool(&config.database_url).await?;
    info!("Database connection established");

    let post_repository: Arc<dyn PostRepository> = Arc::new(SeaOrmPostRepository::new(db));
    let post_use_case = Arc::new(PostUseCase::new(post_repository));

    let app_state = AppState { post_use_case };
    let app = create_routes(app_state).layer(TraceLayer::new_for_http());

    let bind_addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server running at http://{}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
