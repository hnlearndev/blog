// Declare module structure for the server-side application

// Database modules
pub mod db;

// Model modules
pub mod models;

// Handler modules
pub mod handlers;

// Service modules
pub mod services;

// Repository modules
pub mod repositories;

// Route modules
pub mod routes;

// Middleware modules
pub mod middleware;

// Import necessary crates and modules
use crate::server::db::{config, pool, state::AppState};
use crate::server::middleware::request_logger::request_logger;
use crate::server::middleware::cors::cors_layer;
use crate::server::middleware::security_headers::security_headers;
use crate::server::routes::subscriber::subscriber_routes;
use axum::{Router, middleware::from_fn};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

/// Main server run function - called by main.rs
#[cfg(feature = "ssr")]
pub async fn run() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

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
        .layer(from_fn(request_logger))
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
    log!("🚀 Server starting on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
