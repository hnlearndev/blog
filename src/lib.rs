pub mod app;
pub mod client;

#[cfg(feature = "ssr")]
pub mod server;

pub mod shared;

// Use wee_alloc as the global allocator for WASM to reduce size
#[cfg(feature = "hydrate")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
