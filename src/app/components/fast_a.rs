use leptos::prelude::*;
use leptos_router::components::ToHref;

/// A progressively enhanced anchor component.
/// Relies on Leptos 0.8 router's global click handler for client-side navigation —
/// no custom event handler needed.
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
    let path = href.to_href()();

    view! {
        <a href=path target=target class=class.map(Oco::from)>
            {children()}
        </a>
    }
}
