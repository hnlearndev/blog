include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::{components::ToHref, hooks::use_navigate};

#[derive(Debug)]
pub struct PostMetadata {
    pub id: String,
    pub date: String,
    pub title: String,
}

pub fn get_post_metadata_list() -> Vec<PostMetadata> {
    POSTS
        .iter()
        .map(|&(id, date, title, _content)| PostMetadata {
            id: id.to_string(),
            date: date.to_string(),
            title: title.to_string(),
        })
        .collect()
}

pub fn get_post(path: String) -> Option<(PostMetadata, String)> {
    POSTS
        .iter()
        .filter_map(|(id, date, title, content)| {
            if id == &path {
                Some((
                    PostMetadata {
                        id: id.to_string(),
                        date: date.to_string(),
                        title: title.to_string(),
                    },
                    content.to_string(),
                ))
            } else {
                None
            }
        })
        .next()
}

#[component]
pub fn FastA<H>(
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
