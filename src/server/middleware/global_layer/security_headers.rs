use axum::{
    body::Body,
    http::{
        Request,
        header::{HeaderName, HeaderValue},
    },
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use std::env;

// Generate a nonce for CSP inline scripts
fn generate_csp_nonce() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}

pub async fn security_headers(req: Request<Body>, next: Next) -> Response {
    let mut res = next.run(req).await;
    
    // Generate nonce for this response
    let nonce = generate_csp_nonce();
    
    // Check if we're in production mode
    let is_production = env::var("LEPTOS_ENV")
        .unwrap_or_else(|_| "DEV".to_string())
        .to_uppercase() == "PROD";

    // Build CSP policy based on environment
    let csp_policy = if is_production {
        format!(
            "default-src 'self'; \
             script-src 'self' 'nonce-{}' 'wasm-unsafe-eval'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: https:; \
             font-src 'self' data:; \
             connect-src 'self'; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self'; \
             upgrade-insecure-requests;",
            nonce
        )
    } else {
        // More permissive for development
        "default-src 'self' ws: wss:; \
             script-src 'self' 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: https:; \
             font-src 'self' data:; \
             connect-src 'self' ws: wss:;".to_string()
    };

    // Security headers
    let headers = [
        ("content-security-policy", csp_policy.as_str()),
        ("x-content-type-options", "nosniff"),
        ("x-frame-options", "DENY"),
        ("x-xss-protection", "1; mode=block"),
        ("referrer-policy", "strict-origin-when-cross-origin"),
        ("permissions-policy", "camera=(), microphone=(), geolocation=(), interest-cohort=()"),
        ("x-permitted-cross-domain-policies", "none"),
    ];

    // Add HSTS header only in production
    if is_production {
        res.headers_mut().insert(
            HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        );
    }

    for (name, value) in headers.iter() {
        if let Ok(header_value) = HeaderValue::from_str(value) {
            res.headers_mut().insert(
                HeaderName::from_static(name),
                header_value,
            );
        }
    }
    
    // Store nonce in response extensions for use in templates
    res.extensions_mut().insert(nonce);

    res
}
