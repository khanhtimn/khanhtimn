use topcoat::{
    Result,
    view::{component, view},
};

#[component]
pub async fn profile_header() -> Result {
    view! {
        <header class="space-y-4">
            <div>
                <h1
                    class="text-3xl sm:text-4xl font-bold tracking-tight text-stone-900 dark:text-stone-100 font-serif"
                >
                    "Quang Khánh"
                </h1>
                <div
                    class="mt-1.5 space-y-0.5 text-xs sm:text-sm text-stone-500 dark:text-stone-400 font-serif leading-snug"
                >
                    <p>"HUST IT1 (2023-2026)"</p>
                    <p>"CLC EK17 (2020-2022)"</p>
                </div>
            </div>

            <p
                class="text-stone-800 dark:text-stone-200 text-sm sm:text-base leading-relaxed font-serif max-w-xl"
            >
                "I am working on real-time & high-performance runtimes, and games!"
            </p>

            <div
                class="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs sm:text-sm text-stone-800 dark:text-stone-300 font-serif pt-1"
            >
                <div>
                    <span class="text-stone-500 dark:text-stone-400">"Email: "</span>
                    <a
                        href="mailto:khanhlcbb@gmail.com"
                        class="underline underline-offset-4 decoration-stone-300 hover:decoration-stone-800 dark:decoration-stone-600 dark:hover:decoration-stone-200 transition-colors"
                    >
                        "khanhlcbb@gmail.com"
                    </a>
                </div>
                <div>
                    <span class="text-stone-500 dark:text-stone-400">"GitHub: "</span>
                    <a
                        href="https://github.com/khanhtimn"
                        target="_blank"
                        rel="noreferrer"
                        class="underline underline-offset-4 decoration-stone-300 hover:decoration-stone-800 dark:decoration-stone-600 dark:hover:decoration-stone-200 transition-colors"
                    >
                        "@khanhtimn"
                    </a>
                </div>
            </div>
        </header>
    }
}
