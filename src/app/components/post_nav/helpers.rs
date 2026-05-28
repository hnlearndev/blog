//! Shared helper components and data for post navigation.

use crate::app::components::fast_a::FastA;
use crate::app::helpers::ContentMetadata;
use icons::{ArrowLeft, ArrowRight, ArrowUp};
use leptos::prelude::*;
use leptos::web_sys;

/// Direction for post navigation link.
pub(super) enum PostLinkDirection {
    Prev,
    Next,
}

/// Previous and next post data for navigation.
#[derive(Clone)]
pub struct PostNavData {
    pub prev: Option<ContentMetadata>,
    pub next: Option<ContentMetadata>,
}

/// Back to top button with smooth scroll behavior.
#[component]
pub(super) fn BackToTop() -> impl IntoView {
    view! {
        <a
            href="#"
            on:click=|ev| {
                ev.prevent_default();
                if let Some(win) = leptos::prelude::window()
                    .document()
                    .and_then(|d| d.default_view())
                {
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(0.0);
                    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                    win.scroll_to_with_scroll_to_options(&opts);
                }
            }
            aria-label="Back to top"
            class="post-nav-top"
        >
            <ArrowUp class="size-4 pointer-events-none" />
        </a>
    }
}

/// Private helper: renders a post navigation link with a pre-built title view.
pub(super) fn render_post_link(
    post_id: String,
    route_prefix: String,
    direction: PostLinkDirection,
    title_view: AnyView,
) -> impl IntoView {
    let href = format!("{}/{}", route_prefix, post_id);
    let is_next = matches!(direction, PostLinkDirection::Next);
    let link_class = if is_next {
        "post-nav-link post-nav-next"
    } else {
        "post-nav-link"
    };

    view! {
        <FastA href=href class=link_class>
            {if is_next {
                view! {
                    {title_view}
                    <ArrowRight class="size-4 pointer-events-none" />
                }
                    .into_any()
            } else {
                view! {
                    <ArrowLeft class="size-4 pointer-events-none" />
                    {title_view}
                }
                    .into_any()
            }}
        </FastA>
    }
}
