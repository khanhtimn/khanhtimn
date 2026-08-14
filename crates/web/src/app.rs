use crate::components::*;
use topcoat::{
    Result,
    router::{RouterBuilder, RouterBuilderDiscoverExt, layout, page},
    view::{View, view},
};

pub fn router() -> RouterBuilder {
    topcoat::router::module_router!().discover()
}

#[layout]
async fn root_layout(slot: Result<View>) -> Result {
    view! {
        <!DOCTYPE html>
        <html
            lang="en"
            class="scroll-smooth bg-[#FAF9F5] text-stone-900 dark:bg-[#121110] dark:text-[#EAE6DF] antialiased selection:bg-stone-200 dark:selection:bg-stone-800"
        >
            doc_head()
            audio_unlock_script()
            game_sandbox()
            <body
                class="min-h-screen flex flex-col justify-between font-serif relative"
            >
                (slot?)
            </body>
        </html>
    }
}

#[page]
async fn home() -> Result {
    view! {
        <main
            class="w-full max-w-2xl px-6 sm:px-10 lg:px-16 py-12 sm:py-16 lg:py-24 flex-1 flex flex-col justify-between relative z-10"
        >
            <div class="space-y-12 mb-16">
                profile_header()
                blog_posts_section()
                project_section()
            </div>

            site_footer()
        </main>
    }
}
