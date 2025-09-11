use crate::server::handlers::status::status_badge;
use crate::server::services::status::StatusData;
use axum::{Router, routing::get};

pub fn status_routes(status: StatusData) -> Router {
    Router::new()
        .route("/status-badge", get(status_badge))
        .with_state(status)
}
