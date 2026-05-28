use axum::http::StatusCode;
// Declare module structure for the server-side application
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;

// Import necessary crates and modules
use crate::app::shell;
use crate::server::{
    middleware::global_layer::{cors_layer, security_headers},
    models::status::StatusBadge,
    routes::status::status_routes,
    services::status::StatusService,
};
use axum::{Router, middleware::from_fn};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Main server run function - called by main.rs
#[cfg(feature = "ssr")]
pub async fn run() {
    // Shared status state
    let status: Arc<Mutex<StatusBadge>> = Arc::new(Mutex::new(StatusBadge::unknown()));

    // Start periodic status monitor
    StatusService::start_status_monitor(status.clone());

    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().pretty())
        .init();

    // Get Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(crate::app::App);

    // Build the Axum router with Leptos integration
    let app = Router::new()
        // Apply middleware layers (outermost first)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .layer(from_fn(security_headers))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone())
        .merge(status_routes(status));

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
