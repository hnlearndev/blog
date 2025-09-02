use super::icons::*;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::{components::ToHref, hooks::use_navigate};

#[component]
pub fn Nav() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    let close_mobile_menu = move |_| {
        set_mobile_menu_open.set(false);
    };

    view! {
        <nav class="nav-enhanced">
            // Left side - Brand/Logo
            <div class="nav-brand">
                <FastA href="/" class="brand-link">
                    <img src="/favicon.ico" alt="Brand Logo" width="32" height="32" style="vertical-align: middle; margin-right: 0.5rem;" />
                    <strong>"Willian Nguyen"</strong>
                </FastA>
            </div>

            // Right side - Desktop Navigation
            <div class="nav-desktop">
                <ul class="nav-links">
                    <li class="nav-item">
                        <FastA href="/" class="nav-link">
                            <div class="nav-icon"><HomeIcon /></div>
                            <span>"Home"</span>
                        </FastA>
                    </li>
                    <li class="nav-item">
                        <FastA href="/about" class="nav-link">
                            <div class="nav-icon"><UserIcon /></div>
                            <span>"About"</span>
                        </FastA>
                    </li>
                    <li class="nav-item">
                        <FastA href="/blog" class="nav-link">
                            <div class="nav-icon"><BlogIcon /></div>
                            <span>"Blog"</span>
                        </FastA>
                    </li>
                    <li class="nav-item">
                        <FastA href="/contact" class="nav-link">
                            <div class="nav-icon"><ContactIcon /></div>
                            <span>"Contact"</span>
                        </FastA>
                    </li>
                </ul>

                // Social Media Icons
                <div class="nav-social">
                    <a href="https://github.com/hnlearndev" target="_blank" class="social-link" title="GitHub">
                        <div class="social-icon"><GitHubIcon /></div>
                    </a>
                    // <a href="https://twitter.com/your-username" target="_blank" class="social-link" title="Twitter">
                    //     <div class="social-icon"><TwitterIcon /></div>
                    // </a>
                    <a href="https://www.linkedin.com/in/hieunthello/" target="_blank" class="social-link" title="LinkedIn">
                        <div class="social-icon"><LinkedInIcon /></div>
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
            <div
                class=move || format!("mobile-menu {}",
                    if mobile_menu_open.get() { "mobile-menu-open" } else { "" }
                )
            >
                <div class="mobile-menu-content">
                    <ul class="mobile-nav-links">
                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/" class="mobile-nav-link">
                                    <div class="mobile-nav-icon"><HomeIcon /></div>
                                    <span>"Home"</span>
                                </FastA>
                            </div>
                        </li>
                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/about" class="mobile-nav-link">
                                    <div class="mobile-nav-icon"><UserIcon /></div>
                                    <span>"About"</span>
                                </FastA>
                            </div>
                        </li>
                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/blog" class="mobile-nav-link">
                                    <div class="mobile-nav-icon"><BlogIcon /></div>
                                    <span>"Blog"</span>
                                </FastA>
                            </div>
                        </li>
                        <li class="mobile-nav-item">
                            <div on:click=close_mobile_menu>
                                <FastA href="/contact" class="mobile-nav-link">
                                    <div class="mobile-nav-icon"><ContactIcon /></div>
                                    <span>"Contact"</span>
                                </FastA>
                            </div>
                        </li>
                    </ul>

                    // Social Media in Mobile Menu
                    <div class="mobile-nav-social">
                        <a href="https://github.com/hnlearndev" target="_blank" class="mobile-social-link">
                            <div class="mobile-social-icon"><GitHubIcon /></div>
                            <span>"GitHub"</span>
                        </a>
                        // <a href="https://twitter.com/your-username" target="_blank" class="mobile-social-link">
                        //     <div class="mobile-social-icon"><TwitterIcon /></div>
                        //     <span>"Twitter"</span>
                        // </a>
                        <a href="https://www.linkedin.com/in/hieunthello/" target="_blank" class="mobile-social-link">
                            <div class="mobile-social-icon"><LinkedInIcon /></div>
                            <span>"LinkedIn"</span>
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn FastA<H>(
    href: H,
    #[prop(optional)] target: Option<&'static str>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let navigate = use_navigate();
    let path = href.to_href()();

    fn is_left_click(event: &MouseEvent) -> bool {
        event.button() == 0
            && !event.meta_key()
            && !event.ctrl_key()
            && !event.shift_key()
            && !event.alt_key()
    }

    view! {
        <a
            href=path.clone()
            target=target
            class=class.map(Oco::from)
            on:mousedown=move |event| {
                if is_left_click(&event) {
                    event.prevent_default();
                    navigate(&path, Default::default());
                }
            }
        >
            {children()}
        </a>
    }
}
