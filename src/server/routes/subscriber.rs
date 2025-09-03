use crate::server::db::state::AppState;
use crate::server::handlers::subscriber::subscribe_handler;
use crate::server::middleware::governor::subscriber_governor;
use crate::server::middleware::throttle::subscriber_throttle;
use crate::server::middleware::csrf::csrf;
use axum::middleware::from_fn;
use axum::{Router, routing::post};

pub fn subscriber_routes() -> Router<AppState> {
    Router::new()
        .route("/api/subscribe", post(subscribe_handler))
        .layer(subscriber_governor())
        .layer(from_fn(subscriber_throttle))
        .layer(from_fn(csrf))
}
