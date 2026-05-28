//! Desktop post navigation: horizontal layout [← Previous] [↑ Top] [Next →].

use crate::app::helpers::{ContentMetadata, wrap_words};
use leptos::prelude::*;

use super::helpers::{BackToTop, PostLinkDirection, PostNavData, render_post_link};

/// Desktop post navigation — shown at >768px viewport width.
#[component]
pub fn DesktopPostNavigation(
    /// Navigation data (prev/next posts)
    data: PostNavData,
    /// Route prefix for links (e.g., "/posts")
    route_prefix: String,
) -> impl IntoView {
    let prev = data.prev;
    let next = data.next;

    view! {
        <nav class="post-nav post-nav-desktop" aria-label="Post navigation">
            // Left - Previous post
            <div style="flex: 1;">
                {if let Some(p) = prev {
                    view! {
                        <PostLink
                            post=p
                            route_prefix=route_prefix.clone()
                            direction=PostLinkDirection::Prev
                        />
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            // Center - Back to top
            <BackToTop />

            // Right - Next post
            <div style="flex: 1; text-align: right;">
                {if let Some(n) = next {
                    view! {
                        <PostLink
                            post=n
                            route_prefix=route_prefix.clone()
                            direction=PostLinkDirection::Next
                        />
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </nav>
    }
}

// ==============================================================================
// PRIVATE COMPONENTS
// ==============================================================================

/// Desktop post navigation link with multi-line wrapped titles.
#[component]
fn PostLink(
    /// The post to link to
    post: ContentMetadata,
    /// Route prefix (e.g., "/posts")
    route_prefix: String,
    /// Direction: Prev or Next
    direction: PostLinkDirection,
) -> impl IntoView {
    let (label, _) = match direction {
        PostLinkDirection::Prev => ("Previous", false),
        PostLinkDirection::Next => ("Next", true),
    };
    let title_lines = wrap_words(&post.title, 5);
    let title_view = view! {
        <div>
            <div style="font-size: 0.75rem; opacity: 0.6;">{label}</div>
            <div style="font-size: 0.875rem; font-weight: 500;">
                {title_lines
                    .into_iter()
                    .enumerate()
                    .map(|(i, line)| {
                        if i > 0 {
                            view! {
                                <span>
                                    <br />
                                    {line}
                                </span>
                            }
                                .into_any()
                        } else {
                            view! { <span>{line}</span> }.into_any()
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_view()}
            </div>
        </div>
    }
    .into_any();

    render_post_link(post.id, route_prefix, direction, title_view)
}
