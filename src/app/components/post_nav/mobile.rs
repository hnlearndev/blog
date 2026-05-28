//! Mobile post navigation: vertical stack layout [↑ Top] [← Previous] [Next →].

use crate::app::helpers::{ContentMetadata, truncate_words};
use leptos::prelude::*;

use super::helpers::{BackToTop, PostLinkDirection, PostNavData, render_post_link};

/// Mobile post navigation — shown at ≤768px viewport width.
#[component]
pub fn MobilePostNavigation(
    /// Navigation data (prev/next posts)
    data: PostNavData,
    /// Route prefix for links (e.g., "/posts")
    route_prefix: String,
) -> impl IntoView {
    let prev = data.prev;
    let next = data.next;

    view! {
        <nav class="post-nav post-nav-mobile" aria-label="Post navigation">
            // Top - Back to top button (centered)
            <BackToTop />

            // Prev + Next on same row
            <div class="post-nav-mobile-row">
                // Previous post (left)
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

                // Next post (right)
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
            </div>
        </nav>
    }
}

// ==============================================================================
// PRIVATE COMPONENTS
// ==============================================================================

/// Mobile post navigation link with single-line truncated titles.
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
    let truncated = truncate_words(&post.title, 3);
    let title_view = view! {
        <div>
            <div style="font-size: 0.75rem; opacity: 0.6;">{label}</div>
            <div style="font-size: 0.875rem; font-weight: 500;">{truncated}</div>
        </div>
    }
    .into_any();

    render_post_link(post.id, route_prefix, direction, title_view)
}
