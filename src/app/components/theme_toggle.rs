use super::icons::{MoonIcon, SunIcon};
use leptos::{ev::MouseEvent, prelude::*};

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{Document, HtmlElement, js_sys, window};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    // false = light mode, true = dark mode
    let (is_dark, set_is_dark) = signal(false);

    // Initialize theme based on system preference or stored preference
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Check for stored theme preference first
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(stored_theme)) = storage.get_item("theme") {
                        let is_dark_stored = stored_theme == "dark";
                        set_is_dark.set(is_dark_stored);
                        apply_theme(&document, is_dark_stored);
                        return;
                    }
                }

                // Fall back to system preference
                if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                    let system_prefers_dark = media_query.matches();
                    set_is_dark.set(system_prefers_dark);
                    apply_theme(&document, system_prefers_dark);
                }
            }
        }
    });

    let toggle_theme = move |_: MouseEvent| {
        let new_is_dark = !is_dark.get();
        set_is_dark.set(new_is_dark);

        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    apply_theme(&document, new_is_dark);

                    // Store preference
                    if let Ok(Some(storage)) = window.local_storage() {
                        let theme = if new_is_dark { "dark" } else { "light" };
                        let _ = storage.set_item("theme", theme);
                    }
                }

                // Notify nav component to update logo
                if let Ok(nav_toggle) = js_sys::Reflect::get(&window, &"__nav_theme_toggle".into())
                {
                    if let Ok(function) = nav_toggle.dyn_into::<js_sys::Function>() {
                        let _ = function.call0(&window);
                    }
                }
            }
        }
    };

    view! {
        <button
            class=move || format!(
                "theme-toggle {}",
                if is_dark.get() { "theme-toggle--dark" } else { "theme-toggle--light" }
            )
            on:click=toggle_theme
            title=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
            aria-label=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
        >
            <div class="theme-toggle__track">
                <div class="theme-toggle__thumb">
                    <div class="theme-toggle__icon theme-toggle__icon--sun">
                        <SunIcon />
                    </div>
                    <div class="theme-toggle__icon theme-toggle__icon--moon">
                        <MoonIcon />
                    </div>
                </div>
            </div>
        </button>
    }
}

#[cfg(feature = "hydrate")]
fn apply_theme(document: &Document, is_dark: bool) {
    if let Some(html_element) = document.document_element() {
        if let Ok(html) = html_element.dyn_into::<HtmlElement>() {
            if is_dark {
                let _ = html.set_attribute("data-theme", "dark");
            } else {
                let _ = html.set_attribute("data-theme", "light");
            }
        }
    }
}
