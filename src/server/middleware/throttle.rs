use axum::{http::Request, middleware::Next, response::Response};
use std::time::Duration;
use tokio::time::sleep;

// Add a fixed delay to requests for throttling purposes
pub async fn subscriber_throttle(req: Request<axum::body::Body>, next: Next) -> Response {
    sleep(Duration::from_millis(500)).await; // 500ms delay
    next.run(req).await
}

// Add a fixed delay to requests for throttling purposes
pub async fn status_badge_throttle(req: Request<axum::body::Body>, next: Next) -> Response {
    sleep(Duration::from_millis(1000)).await; // 1000ms delay
    next.run(req).await
}
