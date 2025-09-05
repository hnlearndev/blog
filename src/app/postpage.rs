use super::helpers::get_post;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

// NOTE: This component is rendered via a Leptos router route, not an HTTP API route.
// All post data is accessed locally, NOT  fetched from a backend API.
// If you add an Axum backend in the future, you can switch to fetching data via HTTP.
#[component]
pub fn PostPage() -> impl IntoView {
    // Get the current route parameters eg: /posts/:id
    let params = use_params_map();

    // Extract the "id" parameter from the route
    let id = move || params.read().get("id").unwrap_or_default();

    // Fetch the post metadata and HTML content based on the extracted id
    let (metadata, html) = get_post(id()).unwrap();

    // Render the post page with title and content
    view! {
        <Title text=metadata.title />
        <div inner_html=html />
    }
}
