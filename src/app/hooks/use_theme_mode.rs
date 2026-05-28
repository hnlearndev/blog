use leptos::prelude::*;

#[cfg(feature = "hydrate")]
use web_sys::{Storage, window};

#[derive(Debug, Clone, Copy)]
pub struct ThemeMode {
    state: RwSignal<bool>,
}

#[allow(dead_code)]
const LOCALSTORAGE_KEY: &str = "darkmode";

/// Hook to access the dark mode context
///
/// Returns the ThemeMode instance from context for easy access
pub fn use_theme_mode() -> ThemeMode {
    expect_context::<ThemeMode>()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl ThemeMode {
    #[must_use]
    /// Initializes a new ThemeMode instance.
    pub fn init() -> Self {
        let theme_mode = Self {
            state: RwSignal::new(false),
        };

        provide_context(theme_mode);

        // Use Effect to handle browser-only initialization
        Effect::new(move |_| {
            let initial = Self::get_storage_state().unwrap_or(Self::prefers_dark_mode());
            theme_mode.state.set(initial);
            Self::apply_theme(initial);
        });

        // Apply theme whenever state changes
        Effect::new(move |_| {
            let is_dark = theme_mode.state.get();
            Self::apply_theme(is_dark);
        });

        theme_mode
    }

    pub fn toggle(&self) {
        self.state.update(|state| {
            *state = !*state;
            Self::set_storage_state(*state);
        });
    }

    pub fn set_dark(&self) {
        self.set(true);
    }

    pub fn set_light(&self) {
        self.set(false);
    }

    /// - `dark`: Set to `true` for dark mode, and `false` for light mode.
    pub fn set(&self, dark: bool) {
        self.state.set(dark);
        Self::set_storage_state(dark);
    }

    #[must_use]
    pub fn get(&self) -> bool {
        self.state.get()
    }

    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.state.get()
    }

    #[must_use]
    pub fn is_light(&self) -> bool {
        !self.state.get()
    }

    /* ========================================================== */
    /*                     ✨ FUNCTIONS ✨                        */
    /* ========================================================== */

    /// Retrieves the local storage object, if available.
    #[cfg(feature = "hydrate")]
    fn get_storage() -> Option<Storage> {
        window()?.local_storage().ok().flatten()
    }

    /// Retrieves the dark mode state from local storage, if available.
    #[cfg(feature = "hydrate")]
    fn get_storage_state() -> Option<bool> {
        Self::get_storage()
            .and_then(|storage| storage.get(LOCALSTORAGE_KEY).ok())
            .flatten()
            .and_then(|entry| entry.parse::<bool>().ok())
    }

    /// Checks whether the user's system prefers dark mode based on media queries.
    #[cfg(feature = "hydrate")]
    fn prefers_dark_mode() -> bool {
        window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map(|media| media.matches())
            .unwrap_or_default()
    }

    /// Stores the dark mode state in local storage.
    #[cfg(feature = "hydrate")]
    fn set_storage_state(state: bool) {
        if let Some(storage) = Self::get_storage() {
            storage
                .set(LOCALSTORAGE_KEY, state.to_string().as_str())
                .ok();
        }
    }

    #[cfg(not(feature = "hydrate"))]
    fn get_storage_state() -> Option<bool> {
        None
    }

    #[cfg(not(feature = "hydrate"))]
    fn prefers_dark_mode() -> bool {
        false
    }

    #[cfg(not(feature = "hydrate"))]
    fn set_storage_state(_state: bool) {}

    /// Applies the theme to the document element
    #[cfg(feature = "hydrate")]
    fn apply_theme(is_dark: bool) {
        use wasm_bindgen::JsCast;
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(el) = document.document_element() {
                    if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                        let _ = html
                            .set_attribute("data-theme", if is_dark { "dark" } else { "light" });
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "hydrate"))]
    fn apply_theme(_is_dark: bool) {}
}
