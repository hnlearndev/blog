pub mod routes {
    pub mod routes;
    pub mod auth;
    pub mod posts;
}
pub mod models {
    pub mod models;
    pub mod user;
    pub mod post;
}
pub mod db {
    pub mod db;
    pub mod pool;
    pub mod migrations;
}
pub mod middleware {
    pub mod middleware;
    pub mod auth;
    pub mod logging;
}
pub mod error;
pub mod config;
pub mod state;

/// Main server run function - called by main.rs
#[cfg(feature = "ssr")]
pub async fn run() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos::logging::log;
    
    // Get Leptos configuration
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(crate::app::App);
    
    // Build the Axum router with Leptos integration
    let app = Router::new()
        .leptos_routes(
            &leptos_options, 
            routes, 
            {
                let leptos_options = leptos_options.clone();
                move || crate::app::shell(leptos_options.clone())
            }
        )
        .fallback(leptos_axum::file_and_error_handler(crate::app::shell))
        .with_state(leptos_options);

    // Start the server
    log!("🚀 Server starting on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
