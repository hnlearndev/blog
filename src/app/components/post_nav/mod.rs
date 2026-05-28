//! Post navigation module: previous/next links with back-to-top arrow.
//!
//! Provides responsive desktop and mobile layouts following the nav/footer pattern.

mod desktop;
mod helpers;
mod mobile;

pub use desktop::DesktopPostNavigation;
pub use helpers::PostNavData;
pub use mobile::MobilePostNavigation;

use leptos::prelude::*;

/// Post navigation component — composes desktop and mobile variants.
#[component]
pub fn PostNavigation(
    /// Navigation data (prev/next posts)
    data: PostNavData,
    /// Route prefix for links (e.g., "/posts")
    route_prefix: String,
) -> impl IntoView {
    let desktop_data = data.clone();
    let mobile_data = data;

    view! {
        <DesktopPostNavigation data=desktop_data route_prefix=route_prefix.clone() />
        <MobilePostNavigation data=mobile_data route_prefix=route_prefix />
    }
}
