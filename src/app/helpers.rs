use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::params::ParamsMap;

#[derive(Debug, Clone)]
pub struct ContentMetadata {
    pub id: String,
    pub date: String,
    pub title: String,
}

pub fn get_content_metadata_list(data: &[(&str, &str, &str, &str)]) -> Vec<ContentMetadata> {
    data.iter()
        .map(|&(id, date, title, _content)| ContentMetadata {
            id: id.to_string(),
            date: date.to_string(),
            title: title.to_string(),
        })
        .collect()
}

pub fn get_content(
    data: &[(&str, &str, &str, &str)],
    path: &str,
) -> Option<(ContentMetadata, String)> {
    data.iter()
        .filter_map(|(id, date, title, content)| {
            if *id == path {
                Some((
                    ContentMetadata {
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

// NOTE: This component is rendered via a Leptos router route, not an HTTP API route.
// All post data is accessed locally, NOT  fetched from a backend API.
// If you add an Axum backend in the future, you can switch to fetching data via HTTP.
pub fn render_content_page(data: &[(&str, &str, &str, &str)], params: &Memo<ParamsMap>) -> AnyView {
    let id = params.with(|p| p.get("id").unwrap_or_default());
    match get_content(data, &id) {
        Some((meta, content)) => view! {
            <Title text=meta.title.clone() />
            <ContentPage metadata=meta content=content />
        }
        .into_any(),
        None => view! { <p>"Not found."</p> }.into_any(),
    }
}

#[component]
fn ContentPage(metadata: ContentMetadata, content: String) -> impl IntoView {
    view! {
        <article>
            <small>"Date: " {metadata.date.clone()}</small>
            <br />
            <br />
            <br />
            <div inner_html=content></div>
        </article>
    }
}
