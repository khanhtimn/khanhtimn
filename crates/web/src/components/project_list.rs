use crate::models::PROJECTS;
use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn project_section() -> Result {
    view! {
        <section aria-label="Projects" class="space-y-4 pt-2">
            <h2
                class="text-2xl font-bold font-serif text-stone-900 dark:text-stone-100 tracking-tight"
            >
                "Projects & Research"
            </h2>

            if PROJECTS.is_empty() {
                <p class="text-sm font-serif italic text-stone-500 dark:text-stone-400">
                    "Still compiling with --release... check back soon :P"
                </p>
            } else {
                <ul class="space-y-3.5 list-none p-0 m-0">
                    for project in PROJECTS {
                        <li
                            class="group flex flex-col sm:flex-row sm:items-baseline gap-1.5 sm:gap-3 text-sm sm:text-base font-serif"
                        >
                            <span
                                class="text-xs font-mono text-stone-600 dark:text-stone-400 shrink-0 select-none"
                            >
                                "["
                                (project.tech.first().unwrap_or(&"Rust"))
                                "]"
                            </span>
                            <div>
                                <a
                                    href=(project.url)
                                    class="text-stone-900 dark:text-stone-100 underline underline-offset-4 decoration-stone-300 group-hover:decoration-stone-800 dark:decoration-stone-600 dark:group-hover:decoration-stone-200 transition-colors font-medium"
                                >
                                    (project.name)
                                </a>
                                <p
                                    class="mt-1 text-xs sm:text-sm text-stone-500 dark:text-stone-400 leading-relaxed font-serif"
                                >
                                    (project.description)
                                </p>
                            </div>
                        </li>
                    }
                </ul>
            }
        </section>
    }
}
