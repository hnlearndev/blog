use crate::server::db::state::AppState;
use crate::server::handlers::subscriber::subscribe_handler;
use crate::server::middleware::{
    cache::no_cache_layer, csrf::csrf, governor::subscriber_governor, throttle::subscriber_throttle,
};
use axum::{Router, middleware::from_fn, routing::post};

pub fn subscriber_routes() -> Router<AppState> {
    Router::new()
        .route("/api/subscribe", post(subscribe_handler))
        .layer(no_cache_layer())
        .layer(subscriber_governor())
        .layer(from_fn(subscriber_throttle))
        .layer(from_fn(csrf))
}
