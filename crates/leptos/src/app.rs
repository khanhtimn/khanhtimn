use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[cfg(feature = "hydrate")]
use game_client::init_bevy_app;
#[cfg(feature = "hydrate")]
use leptos_bevy_canvas::prelude::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
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
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/khanhtimn_dev.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">"Welcome to Leptos with Bevy!"</h1>

            <div class="mb-4">
                <button
                    on:click=on_click
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                >
                    "Click Me: "
                    {count}
                </button>
            </div>

            <BevyCanvasWrapper />
        </div>
    }
}

#[component]
fn BevyCanvasWrapper() -> impl IntoView {
    let is_mounted = RwSignal::new(false);

    Effect::new(move |_| {
        is_mounted.set(true);
    });

    view! {
        <div
            class="border rounded-lg overflow-hidden bg-gray-800"
            style:width=format!("{}px", 1280)
            style:height=format!("{}px", 960)
        >
            <Show
                when=move || is_mounted.get()
                fallback=move || {
                    view! {
                        <div class="flex items-center justify-center w-full h-full">
                            <p class="text-gray-400">"Loading Bevy canvas..."</p>
                        </div>
                    }
                }
            >
                {move || {
                    #[cfg(feature = "hydrate")]
                    {
                        view! {
                            <BevyCanvas
                                init=move || init_bevy_app()
                                {..}
                                width=format!("{}", 1280)
                                height=format!("{}", 960)
                            />
                        }
                    }
                }}
            </Show>
        </div>
    }
}
