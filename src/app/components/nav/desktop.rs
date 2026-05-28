use super::helpers::{NAV_LINKS, SocialLinks, handle_link_click, nav_icon};
use crate::app::components::ThemeToggle;
use crate::app::components::fast_a::FastA;
use crate::app::components::ui::separator::{Separator, SeparatorOrientation};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn DesktopNav(
    clicked_links: ReadSignal<HashMap<String, String>>,
    set_clicked_links: WriteSignal<HashMap<String, String>>,
) -> impl IntoView {
    let on_click = handle_link_click(set_clicked_links);

    view! {
        <div class="nav-desktop">
            <NavLinks />

            <Separator
                orientation=SeparatorOrientation::Vertical
                style="height: 24px; min-width: 2px; background-color: currentColor"
            />

            // Theme Toggle
            <div class="nav-theme">
                <ThemeToggle />
            </div>

            <Separator
                orientation=SeparatorOrientation::Vertical
                style="height: 24px; min-width: 2px; background-color: currentColor"
            />

            // Contact/Social Media Icons
            <SocialLinks
                container_class="nav-social"
                link_class="social-link"
                icon_class="social-icon"
                clicked_links=clicked_links
                on_click=on_click
            />
        </div>
    }
}

// ==============================================================================
// PRIVATE COMPONENTS
// ==============================================================================

/// Desktop navigation links — horizontal list.
#[component]
fn NavLinks() -> impl IntoView {
    view! {
        <ul class="nav-links">
            {NAV_LINKS
                .iter()
                .map(|(path, label, id)| {
                    let path = *path;
                    let label = *label;
                    let id = *id;
                    view! {
                        <li class="nav-item">
                            <FastA href=path class="nav-link">
                                <div class="nav-icon">{nav_icon(id)}</div>
                                <span>{label}</span>
                            </FastA>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
