//! Mobile footer: vertical stack layout (copyright centered on top, newsletter below).

use leptos::prelude::*;

use super::helpers::{Copyright, NewsletterSection};

/// Mobile footer component — shown at ≤768px viewport width.
#[component]
pub fn MobileFooter(
    /// Reactive signal for dark theme state
    is_dark_theme: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="footer-mobile">
            <Copyright is_dark_theme=is_dark_theme style="justify-content: center;" />
            <NewsletterSection
                container_class="newsletter-mobile"
                label_class="newsletter-label-mobile"
                form_class="newsletter-form-mobile"
            />
        </div>
    }
}
