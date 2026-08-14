use topcoat::{
    Result,
    asset::asset,
    tailwind,
    view::{component, view},
};

#[component]
pub async fn doc_head(#[default("Quang Khánh")] title: &'static str) -> Result {
    view! {
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>(title)</title>
            <meta name="description" content="Personal blog" />
            <meta name="apple-mobile-web-app-title" content="khanhtimn.dev" />

            topcoat::dev::script()
            topcoat::runtime::script()

            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
            <link
                href="https://fonts.googleapis.com/css2?family=Newsreader:ital,opsz,wght@0,6..72,400..700;1,6..72,400..600&family=JetBrains+Mono:wght@400;500;600&display=swap"
                rel="stylesheet"
            />

            <link rel="stylesheet" href=(tailwind::stylesheet!()) />

            <link
                rel="icon"
                type="image/png"
                href=(asset!("../../../../assets/favicon-96x96.png"))
                sizes="96x96"
            />
            <link
                rel="icon"
                type="image/svg+xml"
                href=(asset!("../../../../assets/favicon.svg"))
            />
            <link rel="shortcut icon" href=(asset!("../../../../assets/favicon.ico")) />
            <link
                rel="apple-touch-icon"
                sizes="180x180"
                href=(asset!("../../../../assets/apple-touch-icon.png"))
            />
            <link rel="manifest" href=(asset!("../../../../assets/site.webmanifest")) />
        </head>
    }
}
