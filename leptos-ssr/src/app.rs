use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::views::pages::{Landing, NotFound};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css" />

        // sets the document title
        <Title text="ESRS" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Landing />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}
