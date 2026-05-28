//! Desktop footer: horizontal layout (copyright left, newsletter right).

use leptos::prelude::*;

use super::helpers::{Copyright, NewsletterSection};

/// Desktop footer component — shown at >768px viewport width.
#[component]
pub fn DesktopFooter(
    /// Reactive signal for dark theme state
    is_dark_theme: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="footer-desktop">
            <Copyright is_dark_theme=is_dark_theme />
            <NewsletterSection form_class="newsletter-form-desktop" />
        </div>
    }
}
