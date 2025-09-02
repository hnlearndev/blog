// Import sub-modules
mod about;
mod home;
mod nav;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

// Function to create the HTML shell for the application
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content="Willian's personal website" />
                <meta name="color-scheme" content="light dark" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pico.min.css"/>

        // sets the document title
        <Title text="Welcome to Willian's tech blog"/>

        // content for this welcome page
        <Router>
            <header class="container">
                <nav::Nav/>
            </header>
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=home::HomePage/>
                    <Route path=StaticSegment("/about") view=about::About/>
                </Routes>
            </main>
        </Router>
    }
}
