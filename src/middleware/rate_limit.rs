use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use tokio::sync::RwLock;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct RateLimitLayer {
    max_requests: usize,
    window: Duration,
}

impl RateLimitLayer {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimit<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimit {
            inner,
            limiter: Arc::new(RateLimiter::new(self.max_requests, self.window)),
        }
    }
}

#[derive(Clone)]
pub struct RateLimit<S> {
    inner: S,
    limiter: Arc<RateLimiter>,
}

#[derive(Debug, Default)]
struct RateLimiter {
    requests: RwLock<Vec<(String, Instant)>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: RwLock::new(Vec::new()),
            max_requests,
            window,
        }
    }

    async fn check(&self, key: &str) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        let cutoff = now - self.window;

        requests.retain(|(k, t)| *t > cutoff && k == key);

        let count = requests.iter().filter(|(k, _)| k == key).count();
        if count < self.max_requests {
            requests.push((key.to_string(), now));
            true
        } else {
            false
        }
    }
}

impl<S> Service<Request<Body>> for RateLimit<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let limiter = self.limiter.clone();
        let mut inner = self.inner.clone();

        let ip = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .split(',')
            .next()
            .unwrap_or("unknown")
            .trim()
            .to_string();

        Box::pin(async move {
            if !limiter.check(&ip).await {
                return Ok((
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded",
                )
                    .into_response());
            }
            inner.call(req).await
        })
    }
}

pub fn rate_limit_layer() -> RateLimitLayer {
    RateLimitLayer::new(100, 60)
}
