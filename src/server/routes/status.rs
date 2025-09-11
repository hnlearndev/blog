use crate::server::handlers::status::status_badge;
use crate::server::middleware::governor::status_badge_governor;
use crate::server::middleware::throttle::status_badge_throttle;
use crate::server::services::status::StatusData;
use axum::{Router, middleware::from_fn, routing::get};

pub fn status_routes(status: StatusData) -> Router {
    Router::new()
        .route(
            "/status-badge",
            get(status_badge)
                .layer(status_badge_governor()) // Rate limiting
                .layer(from_fn(status_badge_throttle)), // Delay
        )
        .with_state(status)
}
