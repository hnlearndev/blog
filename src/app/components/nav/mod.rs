pub mod desktop;
mod helpers;
pub mod mobile;

use crate::app::components::fast_a::FastA;
use crate::app::hooks::use_theme_mode::use_theme_mode;
use desktop::DesktopNav;
use leptos::prelude::*;
use mobile::MobileNav;
use std::collections::HashMap;

#[component]
pub fn Nav() -> impl IntoView {
    let (clicked_links, set_clicked_links) = signal(HashMap::<String, String>::new());

    // Use ThemeMode context for reactive theme state
    let theme_mode = use_theme_mode();
    let is_dark_theme = Signal::derive(move || theme_mode.is_dark());

    view! {
        <nav class="nav-enhanced">
            // Left side - Brand/Logo
            <div class="nav-brand">
                <FastA href="/" class="brand-link">
                    <img
                        src=move || {
                            if is_dark_theme.get() { "/dark_logo.svg" } else { "/light_logo.svg" }
                        }
                        alt="Brand Logo"
                        width="32"
                        height="32"
                        style="vertical-align: middle; margin-right: 0.5rem;"
                    />
                    <strong style="font-size: 1.2rem;">"Willian Nguyen"</strong>
                </FastA>
            </div>

            <DesktopNav clicked_links=clicked_links set_clicked_links=set_clicked_links />

            <MobileNav clicked_links=clicked_links set_clicked_links=set_clicked_links />
        </nav>
    }
}
