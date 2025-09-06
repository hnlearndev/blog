use leptos::*;
use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::{components::ToHref, hooks::use_navigate};

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
