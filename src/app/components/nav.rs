use super::{fast_a::FastA, icons::*, theme_toggle::ThemeToggle};
use leptos::{ev::MouseEvent, prelude::*};
use std::collections::HashMap;

#[component]
pub fn Nav() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);
    let (clicked_links, set_clicked_links) = signal(HashMap::<String, String>::new());

    let toggle_mobile_menu = move |_: MouseEvent| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    let close_mobile_menu = move |_: MouseEvent| {
        set_mobile_menu_open.set(false);
    };

    let handle_link_click = move |link_id: String| {
        move |_: MouseEvent| {
            // Add clicked class immediately
            set_clicked_links.update(|links| {
                links.insert(link_id.clone(), "clicked".to_string());
            });

            // Start reset animation after 500ms
            let link_id_clone1 = link_id.clone();
            set_timeout(
                move || {
                    set_clicked_links.update(|links| {
                        links.insert(link_id_clone1, "reset-animation".to_string());
                    });
                },
                std::time::Duration::from_millis(500),
            );

            // Complete reset after animation finishes
            let link_id_clone2 = link_id.clone();
            set_timeout(
                move || {
                    set_clicked_links.update(|links| {
                        links.remove(&link_id_clone2);
                    });
                },
                std::time::Duration::from_millis(800),
            );
        }
    };

    view! {
        <nav class="nav-enhanced">
            // Left side - Brand/Logo
            <div class="nav-brand">
                <FastA href="/" class="brand-link">
                    <img
                        src="/favicon.ico"
                        alt="Brand Logo"
                        width="32"
                        height="32"
                        style="vertical-align: middle; margin-right: 0.5rem;"
                    />
                    <strong>"Willian Nguyen"</strong>
                </FastA>
            </div>

            // Right side - Desktop Navigation
            <div class="nav-desktop">
                <ul class="nav-links">
                    <li class="nav-item">
                        <FastA href="/posts" class="nav-link">
                            <div class="nav-icon">
                                <BlogIcon />
                            </div>
                            <span>"Blog"</span>
                        </FastA>
                    </li>

                    <li class="nav-item">
                        <FastA href="/poems" class="nav-link">
                            <div class="nav-icon">
                                <PoemIcon />
                            </div>
                            <span>"Poems"</span>
                        </FastA>
                    </li>

            //         <li class="nav-item">
            //             <FastA href="/resume" class="nav-link">
            //                 <div class="nav-icon">
            //                     <ResumeIcon />
            //                 </div>
            //                 <span>"Resume"</span>
            //             </FastA>
            //         </li>
                </ul>

                // Theme Toggle
                <div class="nav-theme">
                    <ThemeToggle />
                </div>

                // Contact/Social Media Icons
                <div class="nav-social">
                    <a
                        href="https://github.com/hnlearndev"
                        target="_blank"
                        class=move || {
                            let click_state = clicked_links.get().get("github").cloned().unwrap_or_default();
                            format!("social-link {}", click_state)
                        }
                        title="GitHub"
                        on:click=handle_link_click("github".to_string())
                    >
                        <div class="social-icon">
                            <GitHubIcon />
                        </div>
                    </a>

                    <a
                        href="https://www.linkedin.com/in/hieunthello/"
                        target="_blank"
                        class=move || {
                            let click_state = clicked_links.get().get("linkedin").cloned().unwrap_or_default();
                            format!("social-link {}", click_state)
                        }
                        title="LinkedIn"
                        on:click=handle_link_click("linkedin".to_string())
                    >
                        <div class="social-icon">
                            <LinkedInIcon />
                        </div>
                    </a>

                    <a
                        href="mailto:hieunt.hello@gmail.com"
                        class=move || {
                            let click_state = clicked_links.get().get("email").cloned().unwrap_or_default();
                            format!("social-link {}", click_state)
                        }
                        title="Email"
                        on:click=handle_link_click("email".to_string())
                    >
                        <div class="social-icon">
                            <ContactIcon />
                        </div>
                    </a>
                </div>
            </div>

            // Mobile Menu Button
            <button
                class="mobile-menu-button"
                on:click=toggle_mobile_menu
                aria-label="Toggle mobile menu"
            >
                <Show when=move || mobile_menu_open.get() fallback=|| view! { <MenuIcon /> }>
                    <CloseIcon />
                </Show>
            </button>

            // Mobile Menu
            <div class=move || {
                format!(
                    "mobile-menu {}",
                    if mobile_menu_open.get() { "mobile-menu-open" } else { "" },
                )
            }>
                <div class="mobile-menu-content">
                    <ul class="mobile-nav-links">
                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/blog" class="mobile-nav-link">
                                    <div class="mobile-nav-icon">
                                        <BlogIcon />
                                    </div>
                                    <span>"Blog"</span>
                                </FastA>
                            </div>
                        </li>

                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/poems" class="mobile-nav-link">
                                    <div class="mobile-nav-icon">
                                        <PoemIcon />
                                    </div>
                                    <span>"Poems"</span>
                                </FastA>
                            </div>
                        </li>

                    //     <li class="mobile-nav-item">
                    //         <div on:click=close_mobile_menu>
                    //             <FastA href="/resume" class="mobile-nav-link">
                    //                 <div class="mobile-nav-icon">
                    //                     <ResumeIcon />
                    //                 </div>
                    //                 <span>"Resume"</span>
                    //             </FastA>
                    //         </div>
                    //     </li>
                    </ul>

                    // Theme Toggle in Mobile Menu
                    <div class="mobile-nav-theme">
                        <ThemeToggle />
                    </div>

                    // Social Media in Mobile Menu
                    <div class="mobile-nav-social">
                        <a
                            href="https://github.com/hnlearndev"
                            target="_blank"
                            class=move || {
                                let click_state = clicked_links.get().get("mobile-github").cloned().unwrap_or_default();
                                format!("mobile-social-link {}", click_state)
                            }
                            on:click=handle_link_click("mobile-github".to_string())
                        >
                            <div class="mobile-social-icon">
                                <GitHubIcon />
                            </div>
                            <span>"GitHub"</span>
                        </a>
                        <a
                            href="https://www.linkedin.com/in/hieunthello/"
                            target="_blank"
                            class=move || {
                                let click_state = clicked_links.get().get("mobile-linkedin").cloned().unwrap_or_default();
                                format!("mobile-social-link {}", click_state)
                            }
                            on:click=handle_link_click("mobile-linkedin".to_string())
                        >
                            <div class="mobile-social-icon">
                                <LinkedInIcon />
                            </div>
                            <span>"LinkedIn"</span>
                        </a>
                        <a
                            href="mailto:hieunt.hello@gmail.com"
                            class=move || {
                                let click_state = clicked_links.get().get("mobile-email").cloned().unwrap_or_default();
                                format!("mobile-social-link {}", click_state)
                            }
                            on:click=handle_link_click("mobile-email".to_string())
                        >
                            <div class="mobile-social-icon">
                                <ContactIcon />
                            </div>
                            <span>"Email"</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
