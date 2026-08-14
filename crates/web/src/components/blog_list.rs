use crate::models::BLOG_POSTS;
use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn blog_posts_section() -> Result {
    view! {
        <section aria-label="Blog posts" class="space-y-4">
            <h2
                class="text-2xl font-bold font-serif text-stone-900 dark:text-stone-100 tracking-tight"
            >
                "Blogs"
            </h2>

            if BLOG_POSTS.is_empty() {
                <p class="text-sm font-serif italic text-stone-500 dark:text-stone-400">
                    "Hopefully some soon!"
                </p>
            } else {
                <ul class="space-y-4 list-none p-0 m-0">
                    for post in BLOG_POSTS {
                        <li
                            class="group flex flex-col sm:flex-row sm:items-baseline gap-1.5 sm:gap-3 text-sm sm:text-base font-serif"
                        >
                            <span
                                class="text-xs sm:text-sm font-mono text-stone-600 dark:text-stone-400 shrink-0 select-none"
                            >
                                "("
                                (post.date)
                                ")"
                            </span>
                            <div class="space-y-1">
                                <div class="flex flex-wrap items-center gap-2">
                                    <a
                                        href=(format!("#{}", post.slug))
                                        class="text-stone-900 dark:text-stone-100 underline underline-offset-4 decoration-stone-300 group-hover:decoration-stone-800 dark:decoration-stone-600 dark:group-hover:decoration-stone-200 transition-colors font-medium"
                                    >
                                        (post.title)
                                    </a>
                                </div>
                                <p
                                    class="text-xs sm:text-sm text-stone-500 dark:text-stone-400 leading-relaxed font-serif"
                                >
                                    (post.summary)
                                </p>
                                <div class="flex flex-wrap items-center gap-1.5 pt-0.5">
                                    for tag in post.tags {
                                        <span
                                            class="text-[11px] font-mono text-stone-600 dark:text-stone-400 bg-stone-100 dark:bg-stone-800/80 px-1.5 py-0.5 rounded border border-stone-200/50 dark:border-stone-700/50"
                                        >
                                            "#"
                                            (tag)
                                        </span>
                                    }
                                </div>
                            </div>
                        </li>
                    }
                </ul>
            }
        </section>
    }
}
