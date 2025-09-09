use axum::{middleware::Next, response::Response};
use std::time::Duration;
use tokio::time::sleep;

pub async fn subscriber_throttle(
    req: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Response {
    sleep(Duration::from_millis(500)).await; // 500ms delay
    next.run(req).await
}
