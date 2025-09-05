mod cors;
mod security_headers;

// Re-export the functions from submodules
pub use cors::cors_layer;
pub use security_headers::security_headers;
