use axum::http::StatusCode;
// Declare module structure for the server-side application
// TODO: Uncomment when re-enabling database functionality
// pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories; // Still needed for status
pub mod routes;
pub mod services;
pub mod utils;

// Import necessary crates and modules
use crate::app::shell;
use crate::server::{
    // TODO: Uncomment when re-enabling database functionality
    // db::{config, pool, state::AppState},
    middleware::global_layer::{cors_layer, security_headers},
    models::status::StatusBadge,
    routes::{status::status_routes}, // subscriber::subscriber_routes - commented out
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

    // TODO: Uncomment when re-enabling database functionality
    // Load environment variables from .env file
    // dotenvy::dotenv().ok();

    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().pretty()) // use .pretty() for dev
        .init();

    // Get Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // TODO: Uncomment when re-enabling database functionality
    // Initialize database pool
    // let database_url = config::get_database_url();
    // let db_pool = pool::init_pool(&database_url)
    //     .await
    //     .expect("Failed to initialize database pool");

    // Create app state
    // let app_state = AppState { db_pool };

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(crate::app::App);

    // Build the Axum router with Leptos integration (subscriber API temporarily disabled)
    let app = Router::new()
        // Apply middleware layers (outermost first)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .layer(from_fn(security_headers))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone())
        // TODO: Uncomment when re-enabling subscriber functionality
        // .merge(subscriber_routes().with_state(app_state))
        .merge(status_routes(status));

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
