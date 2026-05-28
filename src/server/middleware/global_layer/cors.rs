use axum::http::{HeaderValue, Method, header};
use std::env;
use tower_http::cors::{AllowOrigin, CorsLayer};

// Apply globally
pub fn cors_layer() -> CorsLayer {
    let is_production = env::var("LEPTOS_ENV").unwrap_or_default() == "PROD";

    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _request_parts| {
                origin
                    .to_str()
                    .map(|s| {
                        if is_production {
                            // Production: allow specific domains
                            matches!(
                                s,
                                "https://williannguyen.com" | "https://www.williannguyen.com"
                            )
                        } else {
                            // Development: allow localhost
                            s.starts_with("http://localhost") || s.starts_with("http://127.0.0.1")
                        }
                    })
                    .unwrap_or(false)
            },
        ))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
            header::ORIGIN,
        ])
        .allow_credentials(true)
}
