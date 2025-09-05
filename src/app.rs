// Import sub-modules
mod footer;
mod helpers;
mod homepage;
mod icons;
mod nav;
mod postpage;
mod subscribe_form;

// Import necessary crates and modules
use homepage::HomePage;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use postpage::PostPage;

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
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pico.min.css" />
        <Stylesheet id="nav-styles" href="/nav-styles.css" />
        <Stylesheet id="footer-styles" href="/footer-styles.css" />

        <Title text="Welcome to Willian's tech blog" />

        <Router>
            // Navigation bar
            <header class="container">
                <nav::Nav />
            </header>

            // Main content area with routing
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/posts/:id") view=PostPage />
                </Routes>
            </main>

            // Footer section
            <footer class="container">
                <footer::Footer />
            </footer>
        </Router>
    }
}
