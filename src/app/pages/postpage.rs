include!(concat!(env!("OUT_DIR"), "/posts_data.rs"));

use crate::app::components::ContentList;
use crate::app::helpers::{get_content_metadata_list, render_content_page};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[component]
pub fn PostListPage() -> impl IntoView {
    view! {
        <Title text="Willian's blog posts" />
        <h1>"Blog Posts"</h1>
        <PostList />
    }
}

#[component]
pub fn PostList() -> impl IntoView {
    let posts = get_content_metadata_list(POSTS);
    view! { <ContentList items=posts route_prefix="/posts" /> }
}

#[component]
pub fn SinglePostPage() -> impl IntoView {
    let params = use_params_map();
    render_content_page(POSTS, &params)
}
