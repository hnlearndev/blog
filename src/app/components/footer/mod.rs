mod desktop;
mod helpers;
mod mobile;
mod subscribe_form;

use crate::app::hooks::use_theme_mode::use_theme_mode;
use desktop::DesktopFooter;
use leptos::prelude::*;
use mobile::MobileFooter;

#[component]
pub fn Footer() -> impl IntoView {
    let theme_mode = use_theme_mode();
    let is_dark_theme = Signal::derive(move || theme_mode.is_dark());

    view! {
        <footer class="footer-enhanced">
            <div class="container mx-auto px-4">
                <DesktopFooter is_dark_theme=is_dark_theme />
                <MobileFooter is_dark_theme=is_dark_theme />
            </div>
        </footer>
    }
}
