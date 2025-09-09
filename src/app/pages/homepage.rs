use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Willian Nguyen - Home" />
        <h1>"Hello there, 👋"</h1>
        <p>"I am a curious learner with a genuine love for new technologies."</p>
        <p>"Anything that I learn and find interesting, I will write about it on my blog."</p>
    }
}
