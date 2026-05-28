//! Shared helper components for footer: copyright and newsletter section.

use icons::MapPinned;
use leptos::prelude::*;

use super::subscribe_form::SubscribeForm;

/// Copyright block with theme-aware logo.
#[component]
pub(super) fn Copyright(
    /// Reactive signal for dark theme state
    is_dark_theme: Signal<bool>,
    /// Optional additional style for the wrapper div
    #[prop(into, optional)]
    style: String,
) -> impl IntoView {
    view! {
        <div class="copyright-section" style=style>
            <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.5rem;">
                <div style="opacity: 0.7;">
                    <MapPinned class="size-4" />
                </div>
                <span class="copyright-text">"HCMC, Vietnam."</span>
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <img
                    src=move || {
                        if is_dark_theme.get() { "/dark_logo.svg" } else { "/light_logo.svg" }
                    }
                    alt="Brand Logo"
                    class="copyright-logo"
                />
                <span class="copyright-text">"© 2026 All rights reserved"</span>
            </div>
        </div>
    }
}

/// Newsletter section: label + subscribe form.
#[component]
pub(super) fn NewsletterSection(
    /// Optional class for the container div
    #[prop(into, optional)]
    container_class: String,
    /// Optional class for the label
    #[prop(into, optional)]
    label_class: String,
    /// Optional class for the form
    #[prop(into, optional)]
    form_class: String,
    /// Optional style for the container div
    #[prop(into, optional)]
    container_style: String,
    /// Optional style for the label
    #[prop(into, optional)]
    label_style: String,
    /// Optional style for the form row
    #[prop(into, optional)]
    form_row_style: String,
) -> impl IntoView {
    view! {
        <div class=format!("newsletter-container {}", container_class) style=container_style>
            <label for="email" class=format!("newsletter-label {}", label_class) style=label_style>
                "NEWSLETTER SUBSCRIBE"
            </label>
            <div class=format!("newsletter-form-row {}", form_class) style=form_row_style>
                <SubscribeForm />
            </div>
        </div>
    }
}
