include!(concat!(env!("OUT_DIR"), "/poems_data.rs"));

use crate::app::components::ContentList;
use crate::app::helpers::{get_content_metadata_list, render_content_page};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[component]
pub fn PoemListPage() -> impl IntoView {
    view! {
        <Title text="Willian's poem posts" />
        <p> I am not purely technical. I do poetry here and there just to kill time. </p>
        <p> My poems are in Vietnamese, my mother-tounge. Most of them are just witty and sarcastic. They can be emotional sometimes. </p>
        <h1>"Poem Posts"</h1>
        <PoemList />
    }
}

#[component]
pub fn PoemList() -> impl IntoView {
    let posts = get_content_metadata_list(POEMS);
    view! { <ContentList items=posts route_prefix="/poems" /> }
}

#[component]
pub fn SinglePoemPage() -> impl IntoView {
    let params = use_params_map();
    render_content_page(POEMS, &params)
}
