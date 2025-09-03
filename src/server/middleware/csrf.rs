use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use axum_extra::extract::cookie::CookieJar;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

const CSRF_HEADER: &str = "x-csrf-token";
const CSRF_COOKIE: &str = "csrf_token";

// Generate a secure random CSRF token
pub fn generate_csrf_token() -> String {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}

// Middleware to validate CSRF token for state-changing requests
pub async fn csrf(req: Request<Body>, next: Next) -> Response {
    match *req.method() {
        axum::http::Method::POST | axum::http::Method::PUT | axum::http::Method::DELETE => {
            let jar = CookieJar::from_headers(req.headers());
            let token_in_cookie = jar.get(CSRF_COOKIE).map(|c| c.value().to_string());
            let token_in_header = req.headers().get(CSRF_HEADER).and_then(|v| v.to_str().ok());

            if token_in_cookie.is_none()
                || token_in_header.is_none()
                || token_in_cookie.as_deref() != token_in_header
            {
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("CSRF token missing or invalid".into())
                    .unwrap();
            }
        }
        _ => {}
    }
    next.run(req).await
}
