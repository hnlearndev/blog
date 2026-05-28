pub mod app;
pub mod client;

// Re-exports for autoform derive macro (expects crate::hooks and crate::ui paths)
pub use app::hooks;
pub mod ui {
    pub use crate::app::components::ui::auto_form;
    pub use crate::app::components::ui::form;
}

#[cfg(feature = "ssr")]
pub mod server;

pub mod shared;

// Use wee_alloc as the global allocator for WASM to reduce size
#[cfg(feature = "hydrate")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
