// Import sub-modules
pub mod components;
pub mod helpers;
pub mod pages;

// Import necessary crates and modules
use components::{Footer, Nav};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use pages::*;

// Function to create the HTML shell for the application
// Used as the entry point for rendering the HTML shell during server-side rendering (SSR) or hydration.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content="Willian's personal website" />
                <meta name="color-scheme" content="light dark" />
                <link rel="icon" href="/favico.svg" />
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
        <Stylesheet id="leptos" href="/style/pico.min.css" />
        <Stylesheet id="nav-styles" href="/style/nav-styles.css" />
        <Stylesheet id="footer-styles" href="/style/footer-styles.css" />

        <Title text="Welcome to Willian's blog" />

        <Router>
            // Navigation bar
            <header class="container">
                <Nav />
            </header>

            // Main content area with routing
            <main class="container">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    // Posts section
                    <Route path=StaticSegment("/posts") view=PostListPage />
                    <Route path=path!("/posts/:id") view=SinglePostPage />
                    // Poems section
                    <Route path=StaticSegment("/poems") view=PoemListPage />
                    <Route path=path!("/poems/:id") view=SinglePoemPage />
                </Routes>
            </main>

            // Footer section
            <footer class="container">
                <Footer />
            </footer>
        </Router>
    }
}
