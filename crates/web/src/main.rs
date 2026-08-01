use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    router::{Router, RouterBuilderDiscoverExt, page},
    tailwind,
    view::{component, view},
};

#[tokio::main]
async fn main() {
    let router = Router::builder()
        .discover()
        .assets(AssetBundle::load_dir("target/assets").unwrap())
        .build();

    topcoat::start(router).await.unwrap();
}

#[page("/")]
async fn home() -> Result {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                topcoat::dev::script()
                topcoat::runtime::script()
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>
            <body>
                // Sound fix
                <link rel="inline" href=(asset!("../../../assets/game/sound.js")) />
                // The bevy game
                <canvas id="bevy_canvas"></canvas>
                // Load bevy wasm module
                <script
                    type="module"
                    id="bevy_game"
                    data-game=(asset!(
                        "../../../assets/game/game_client.js", rename : "game"
                    ))
                    data-wasm=(asset!(
                        "../../../assets/game/game_client_bg.wasm", rename :
                        "game_client_bg"
                    ))
                >
                    "const s = document.getElementById('bevy_game');"
                    "import(s.dataset.game).then(function(m) {"
                    "    return m.default({ module_or_path: s.dataset.wasm }).then(function() {"
                    "        m.init();"
                    "    });"
                    "}).catch(function(error) {"
                    "    if (!error.message?.includes('Using exceptions for control flow')) {"
                    "        console.error('Game launch error:', error);"
                    "    }"
                    "});"
                </script>
            </body>
        </html>
    }
}