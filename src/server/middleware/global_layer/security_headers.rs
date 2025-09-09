use axum::{
    body::Body,
    http::{
        Request,
        header::{HeaderName, HeaderValue},
    },
    middleware::Next,
    response::Response,
};

pub async fn security_headers(req: Request<Body>, next: Next) -> Response {
    let mut res = next.run(req).await;

    let headers: [(&str, &str); 5] = [
        ("x-content-type-options", "nosniff"),
        ("x-frame-options", "DENY"),
        ("x-xss-protection", "1; mode=block"),
        (
            "strict-transport-security",
            "max-age=31536000; includeSubDomains",
        ),
        ("referrer-policy", "no-referrer"),
        // Add more as needed
    ];

    for (name, value) in headers.iter() {
        res.headers_mut().insert(
            HeaderName::from_static(name),
            HeaderValue::from_static(value),
        );
    }

    res
}
