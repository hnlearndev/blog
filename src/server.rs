// Declare module structure for the server-side application
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

// Import necessary crates and modules
use crate::server::{
    db::{config, pool, state::AppState},
    middleware::global_layer::{cors_layer, security_headers},
    routes::subscriber::subscriber_routes,
};
use axum::{
    Router, 
    middleware::from_fn,
};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};


/// Main server run function - called by main.rs
#[cfg(feature = "ssr")]
pub async fn run() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().pretty()) // use .pretty() for dev
        .init();

    // Get Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Initialize database pool
    let database_url = config::get_database_url();
    let db_pool = pool::init_pool(&database_url)
        .await
        .expect("Failed to initialize database pool");

    // Create app state
    let app_state = AppState { db_pool };

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(crate::app::App);

    // Build the Axum router with Leptos integration and subscriber API
    let app = Router::new()
        // Apply middleware layers (outermost first)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .layer(from_fn(security_headers))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || crate::app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(crate::app::shell))
        .with_state(leptos_options.clone())
        .merge(subscriber_routes().with_state(app_state));

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
