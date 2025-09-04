use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

// Apply globally
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(HeaderValue::from_static("*")) // Replace * with your frontend domain in production
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
