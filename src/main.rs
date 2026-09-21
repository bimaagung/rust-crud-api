use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use rust_crud::application::post::PostUseCase;
use rust_crud::config::Config;
use rust_crud::domain::post::PostRepository;
use rust_crud::infrastructure::database::connection::init_pool;
use rust_crud::infrastructure::database::repositories::SeaOrmPostRepository;
use rust_crud::presentation::http::{AppState, create_routes};

// Entry point — Go: func main() { ... }
// Wire up all layers: config → db → repository → use case → handler → server
#[tokio::main] // Go: no annotation needed; main() is synchronous by default
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logger — Go: log.SetFlags(...) or zerolog/zap setup
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_crud=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load(); // Go: viper.ReadInConfig() or os.Getenv(...)

    info!("Connecting to database...");
    let db = init_pool(&config.database_url).await?; // Go: sql.Open("postgres", dsn)
    info!("Database connection established");

    // Dependency injection — Go: repo := mysql.NewRepo(db); svc := usecase.NewService(repo); h := handler.New(svc)
    let post_repository: Arc<dyn PostRepository> = Arc::new(SeaOrmPostRepository::new(db));
    let post_use_case = Arc::new(PostUseCase::new(post_repository));

    // Build app state and router — Go: r := gin.Default(); r.Use(...); setupRoutes(r, h)
    let app_state = AppState { post_use_case };
    let app = create_routes(app_state).layer(TraceLayer::new_for_http()); // TraceLayer ≈ Gin logger middleware

    // Start HTTP server — Go: http.ListenAndServe(addr, r)
    let bind_addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server running at http://{}", bind_addr);

    axum::serve(listener, app).await?;

    Ok(()) // Go: return nil
}
