use axum::http::header::{CACHE_CONTROL, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

/// Long-term cache for static assets (e.g., images, WASM, CSS, JS)
pub fn static_cache_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000"), // 1 year
    )
}

/// Short-term cache for safe, read-only API responses
pub fn api_cache_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=60"), // 1 minute
    )
}

/// No cache for sensitive or dynamic endpoints (e.g., subscribe, login)
pub fn no_cache_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(CACHE_CONTROL, HeaderValue::from_static("no-store"))
}
