use icons::{FileHeart, Github, Linkedin, Mail, NotebookPen};
use leptos::{ev::MouseEvent, prelude::*};
use std::collections::HashMap;

/// Navigation page links shared between desktop and mobile.
/// (path, label, click_id)
pub(super) const NAV_LINKS: &[(&str, &str, &str)] =
    &[("/posts", "Blog", "posts"), ("/poems", "Poems", "poems")];

/// Social/contact links shared between desktop and mobile.
/// (url, click_id, title, is_external)
pub(super) const SOCIAL_LINKS: &[(&str, &str, &str, bool)] = &[
    ("https://github.com/hnlearndev", "github", "GitHub", true),
    (
        "https://www.linkedin.com/in/hieunthello/",
        "linkedin",
        "LinkedIn",
        true,
    ),
    ("mailto:hieunt.hello@gmail.com", "email", "Email", false),
];

pub(super) fn nav_icon(id: &str) -> impl IntoView {
    match id {
        "posts" => view! { <NotebookPen class="size-5" /> }.into_any(),
        "poems" => view! { <FileHeart class="size-5" /> }.into_any(),
        _ => ().into_any(),
    }
}

pub(super) fn social_icon(id: &str) -> impl IntoView {
    match id {
        "github" => view! { <Github class="size-5" /> }.into_any(),
        "linkedin" => view! { <Linkedin class="size-5" /> }.into_any(),
        "email" => view! { <Mail class="size-5" /> }.into_any(),
        _ => ().into_any(),
    }
}

/// Shared click-animation handler for social links.
pub(super) fn handle_link_click(
    set_clicked_links: WriteSignal<HashMap<String, String>>,
) -> impl Fn(String) -> Box<dyn Fn(MouseEvent)> + Copy {
    move |link_id: String| {
        let set_clicked_links = set_clicked_links;
        Box::new(move |_event: MouseEvent| {
            #[cfg(feature = "hydrate")]
            {
                if let Some(target) = _event.target() {
                    use wasm_bindgen::JsCast;
                    if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                        let _ = element.blur();
                    }
                }
            }

            set_clicked_links.update(|links| {
                links.insert(link_id.clone(), "clicked".to_string());
            });

            let link_id_clone1 = link_id.clone();
            set_timeout(
                move || {
                    set_clicked_links.update(|links| {
                        links.insert(link_id_clone1, "reset-animation".to_string());
                    });
                },
                std::time::Duration::from_millis(200),
            );

            let link_id_clone2 = link_id.clone();
            set_timeout(
                move || {
                    set_clicked_links.update(|links| {
                        links.remove(&link_id_clone2);
                    });
                },
                std::time::Duration::from_millis(400),
            );
        }) as Box<dyn Fn(MouseEvent)>
    }
}

/// Shared social links component.
/// Renders `<a>` items for each entry in [`SOCIAL_LINKS`].
///
/// - `container_class`, `link_class`, `icon_class` — CSS class names for layout variants
/// - `clicked_links` — signal tracking click animation state
/// - `on_click` — click-animation handler factory
/// - `id_prefix` — prefix for click IDs (e.g. `"mobile-"` for mobile variant)
#[component]
pub(super) fn SocialLinks(
    container_class: &'static str,
    link_class: &'static str,
    icon_class: &'static str,
    clicked_links: ReadSignal<HashMap<String, String>>,
    on_click: impl Fn(String) -> Box<dyn Fn(MouseEvent)> + Copy + 'static,
    #[prop(default = "")] id_prefix: &'static str,
) -> impl IntoView {
    view! {
        <div class=container_class>
            {SOCIAL_LINKS
                .iter()
                .map(move |(url, id, title, is_external)| {
                    let url = *url;
                    let id = *id;
                    let title = *title;
                    let target = if *is_external { Some("_blank") } else { None };
                    let click_id = if id_prefix.is_empty() {
                        id.to_string()
                    } else {
                        format!("{id_prefix}{id}")
                    };
                    let click_id_for_class = click_id.clone();
                    view! {
                        <a
                            href=url
                            target=target
                            class=move || {
                                let click_state = clicked_links
                                    .get()
                                    .get(&click_id_for_class)
                                    .cloned()
                                    .unwrap_or_default();
                                format!("{link_class} {}", click_state)
                            }
                            title=title
                            on:click=on_click(click_id)
                        >
                            <div class=icon_class>{social_icon(id)}</div>
                        </a>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
