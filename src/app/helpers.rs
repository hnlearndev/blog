use crate::app::components::ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::app::components::{PostNavData, PostNavigation};
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

// Content is compiled into the binary at build time via build.rs.
// Markdown files are processed and included using `include!(concat!(env!("OUT_DIR"), "/..."))`.
// This provides fast, zero-cost content serving without runtime file I/O or database dependencies.
/// Truncate a string to at most `max_words` words, appending "…" if truncated.
pub fn truncate_words(s: &str, max_words: usize) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() <= max_words {
        s.to_string()
    } else {
        format!("{}…", words[..max_words].join(" "))
    }
}

/// Split a string into lines of at most `max_words` words each.
pub fn wrap_words(s: &str, max_words: usize) -> Vec<String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.chunks(max_words).map(|c| c.join(" ")).collect()
}

/// Compute previous and next posts relative to the current post in the list.
fn get_nav_posts(data: &[(&str, &str, &str, &str)], current_id: &str) -> PostNavData {
    let list = get_content_metadata_list(data);
    let pos = list.iter().position(|m| m.id == current_id);
    // List is reverse-chronological (newest first), so:
    // i+1 = older post = "Previous"
    // i-1 = newer post = "Next"
    let prev = pos.and_then(|i| list.get(i + 1).cloned());
    let next = pos.and_then(|i| {
        if i > 0 {
            Some(list[i - 1].clone())
        } else {
            None
        }
    });
    PostNavData { prev, next }
}

pub fn render_content_page(
    data: &[(&str, &str, &str, &str)],
    params: &Memo<ParamsMap>,
    section_name: &str,
    section_path: &str,
) -> AnyView {
    let section_name = section_name.to_string();
    let section_path = section_path.to_string();
    let data: Vec<(String, String, String, String)> = data
        .iter()
        .map(|(a, b, c, d)| (a.to_string(), b.to_string(), c.to_string(), d.to_string()))
        .collect();
    let params = *params;
    view! {
        {move || {
            let id = params.with(|p| p.get("id").unwrap_or_default());
            let data_refs: Vec<(&str, &str, &str, &str)> = data
                .iter()
                .map(|(a, b, c, d)| (a.as_str(), b.as_str(), c.as_str(), d.as_str()))
                .collect();
            match get_content(&data_refs, &id) {
                Some((meta, content)) => {
                    let title_for_page = meta.title.clone();
                    let title_for_crumb = truncate_words(&meta.title, 5);
                    let date = meta.date.clone();
                    let nav_data = get_nav_posts(&data_refs, &meta.id);
                    let section_path_clone = section_path.clone();
                    let section_name = section_name.clone();
                    let section_path = section_path.clone();
                    view! {
                        <Title text=title_for_page.clone() />
                        <Breadcrumb>
                            <BreadcrumbList>
                                <BreadcrumbItem>
                                    <BreadcrumbLink attr:href="/">"Home"</BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbLink attr:href=section_path>
                                        {section_name}
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbPage>{title_for_crumb}</BreadcrumbPage>
                                </BreadcrumbItem>
                            </BreadcrumbList>
                        </Breadcrumb>
                        <h1 class="text-3xl font-bold mt-4 mb-2">{title_for_page}</h1>
                        <small class="text-gray-600 dark:text-gray-400">"Date: " {date}</small>
                        <hr class="my-6 border-gray-200 dark:border-gray-700" />
                        <article
                            class="prose dark:prose-invert max-w-none"
                            inner_html=content
                        ></article>
                        <PostNavigation data=nav_data route_prefix=section_path_clone />
                    }
                        .into_any()
                }
                None => view! { <p>"Not found."</p> }.into_any(),
            }
        }}
    }
    .into_any()
}
