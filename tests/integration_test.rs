use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use urlslim::{build_router, config::Config, db, Cache, AppState};
use std::sync::Arc;

async fn create_test_app() -> AppState {
    let config = Config {
        host: "0.0.0.0".to_string(),
        port: 3000,
        database_url: ":memory:".to_string(),
        base_url: "http://localhost:3000".to_string(),
        cache_capacity: 100,
        rate_limit_requests: 1000,
        rate_limit_window_secs: 60,
    };

    let db = db::init_db(&config.database_url).await.unwrap();
    db::run_migrations(&db).await.unwrap();

    let cache = Arc::new(Cache::new(config.cache_capacity));

    AppState {
        db,
        config,
        cache,
    }
}

#[tokio::test]
async fn test_health_check() {
    let app = build_router(create_test_app().await);
    
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_link() {
    let app = build_router(create_test_app().await);
    
    let body = json!({
        "url": "https://example.com/test"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/links")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let value: Value = serde_json::from_slice(&body).unwrap();
    
    assert!(value.get("code").is_some());
    assert!(value.get("short_url").is_some());
    assert!(value.get("qr_url").is_some());
}

#[tokio::test]
async fn test_redirect_not_found() {
    let app = build_router(create_test_app().await);
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_invalid_url() {
    let app = build_router(create_test_app().await);
    
    let body = json!({
        "url": "not-a-valid-url"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/links")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
