pub mod config;
pub mod db;
mod error;
pub mod handlers;
pub mod middleware;
mod models;
pub mod services;
mod utils;

use std::sync::Arc;

use axum::Router;
use sqlx::SqlitePool;

pub use config::Config;
pub use error::{AppError, Result};
pub use services::Cache;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub cache: Arc<Cache>,
}

pub fn build_router(state: AppState) -> Router {
    use axum::routing::{get, post};
    
    Router::new()
        .route("/", get(handlers::health))
        .route("/api/v1/links", post(handlers::create_link))
        .route("/:code", get(handlers::redirect))
        .route("/:code/qrcode", get(handlers::qrcode))
        .route("/api/v1/links/:code/stats", get(handlers::stats))
        .with_state(state)
}
