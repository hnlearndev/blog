use super::helpers::{NAV_LINKS, SocialLinks, handle_link_click, nav_icon};
use crate::app::components::ThemeToggle;
use crate::app::components::ui::dropdown_menu::*;
use icons::Menu;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn MobileNav(
    clicked_links: ReadSignal<HashMap<String, String>>,
    set_clicked_links: WriteSignal<HashMap<String, String>>,
) -> impl IntoView {
    let on_click = handle_link_click(set_clicked_links);

    view! {
        <DropdownMenu align=DropdownMenuAlign::Center>
            <DropdownMenuTrigger as_child=true>
                <button class="mobile-menu-button" aria-label="Toggle mobile menu">
                    <Menu class="size-6" />
                </button>
            </DropdownMenuTrigger>

            <DropdownMenuContent class="mobile-dropdown-content">
                <NavLinks />

                <DropdownMenuSeparator style="background-color: currentColor; margin: 0.5rem 0;" />

                // Theme Toggle
                <div class="mobile-nav-theme">
                    <ThemeToggle />
                </div>

                <DropdownMenuSeparator style="background-color: currentColor; margin: 0.5rem 0;" />

                // Social links
                <SocialLinks
                    container_class="mobile-nav-social"
                    link_class="mobile-social-link"
                    icon_class="mobile-social-icon"
                    clicked_links=clicked_links
                    on_click=on_click
                    id_prefix="mobile-"
                />
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

// ==============================================================================
// PRIVATE COMPONENTS
// ==============================================================================

/// Mobile navigation links — stacked vertically using DropdownMenuAction.
#[component]
fn NavLinks() -> impl IntoView {
    view! {
        <DropdownMenuGroup>
            {NAV_LINKS
                .iter()
                .map(|(path, label, id)| {
                    let path = *path;
                    let label = *label;
                    let id = *id;
                    view! {
                        <DropdownMenuAction href=path.to_string()>
                            <div class="mobile-nav-icon">{nav_icon(id)}</div>
                            <span>{label}</span>
                        </DropdownMenuAction>
                    }
                })
                .collect::<Vec<_>>()}
        </DropdownMenuGroup>
    }
}
