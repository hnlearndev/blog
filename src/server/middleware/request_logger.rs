use axum::{middleware::Next, http::Request, response::Response, body::Body};
use tracing::info;
use std::time::Instant;

// Apply globally
pub async fn request_logger(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();
    let res = next.run(req).await;
    let status = res.status().as_u16();
    let duration = start.elapsed().as_millis();
    info!(method = %method, path = %path, status = status, duration_ms = duration, "request completed");
    res
}
