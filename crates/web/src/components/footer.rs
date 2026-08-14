use topcoat::{
    Result,
    asset::asset,
    view::{component, view},
};

#[component]
pub async fn site_footer() -> Result {
    view! {
        <footer
            class="pt-12 pb-8 border-t border-stone-200/60 dark:border-stone-800 text-xs font-mono text-stone-600 dark:text-stone-400"
        >
            <div class="flex items-center gap-2">
                <span>"Buy me a coffee"</span>
                <img
                    src=(asset!("../../../../assets/cute-coffee.svg"))
                    alt="cute-coffee"
                    class="w-6 h-6 inline-block"
                />
            </div>
        </footer>
    }
}
