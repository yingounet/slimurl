mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use config::Config;
pub use error::{AppError, Result};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub cache: Arc<services::Cache>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "urlslim=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting URLSlim...");

    let config = Config::from_env();
    tracing::info!("Configuration loaded: {:?}", config);

    let db = db::init_db(&config.database_url).await?;
    tracing::info!("Database initialized");

    db::run_migrations(&db).await?;
    tracing::info!("Migrations completed");

    let _cleanup_handle = middleware::start_cleanup_task(db.clone());
    tracing::info!("Cleanup task started");

    let cache = Arc::new(services::Cache::new(config.cache_capacity));

    let state = AppState {
        db,
        config: config.clone(),
        cache,
    };

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/api/v1/links", post(handlers::create_link))
        .route("/:code", get(handlers::redirect))
        .route("/:code/qrcode", get(handlers::qrcode))
        .route("/api/v1/links/:code/stats", get(handlers::stats))
        .layer(middleware::rate_limit_layer())
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>;

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}
