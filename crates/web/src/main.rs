use std::path::PathBuf;

use topcoat::router::tower::TowerRoute;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    router::{Methods, Path as RoutePath, Router, RouterBuilderDiscoverExt, page},
    tailwind,
    view::view,
};
use tower_http::services::ServeDir;

/// Resolves the base assets directory from the `ASSETS_DIR` env var,
/// falling back to `<workspace>/assets` in development.
fn assets_dir() -> PathBuf {
    std::env::var("ASSETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("assets"))
                .unwrap_or_else(|| PathBuf::from("assets"))
        })
}

#[tokio::main]
async fn main() -> Result<()> {
    let assets_dir = assets_dir();
    let assets = AssetBundle::load()?;

    // TowerRoute passes the full original URI to the service
    // so ServeDir's base must be the parent
    // of assets_dir so that base/assets/game/... resolves correctly.
    let serve_base = assets_dir.parent().unwrap_or(&assets_dir);

    let router = Router::builder()
        .discover()
        .assets(assets)
        .route(TowerRoute::new(
            Methods::Any,
            RoutePath::new("/assets/game/{*rest}"),
            ServeDir::new(serve_base),
        ))
        .build();

    topcoat::start(router).await?;
    Ok(())
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
                <link
                    rel="icon"
                    type="image/png"
                    href=(asset!("../../../assets/favicon-96x96.png"))
                    sizes="96x96"
                />
                <link
                    rel="icon"
                    type="image/svg+xml"
                    href=(asset!("../../../assets/favicon.svg"))
                />
                <link
                    rel="shortcut icon"
                    href=(asset!("../../../assets/favicon.ico"))
                />
                <link
                    rel="apple-touch-icon"
                    sizes="180x180"
                    href=(asset!("../../../assets/apple-touch-icon.png"))
                />
                <meta name="apple-mobile-web-app-title" content="khanhtimn.dev" />
                <link
                    rel="manifest"
                    href=(asset!("../../../assets/site.webmanifest"))
                />

                <script>
                    // Insert hack to make sound autoplay on Chrome as soon as the user interacts with the tab:
                    // https://developers.google.com/web/updates/2018/11/web-audio-autoplay#moving-forward

                    // the following function keeps track of all AudioContexts and resumes them on the first user
                    // interaction with the page. If the function is called and all contexts are already running,
                    // it will remove itself from all event listeners.
                    "(function () {"
                    // An array of all contexts to resume on the page
                    "   const audioContextList = [];"

                    // An array of various user interaction events we should listen for
                    "   const userInputEventNames = ["
                    "       'click',"
                    "       'contextmenu',"
                    "       'auxclick',"
                    "       'dblclick',"
                    "       'mousedown',"
                    "       'mouseup',"
                    "       'pointerup',"
                    "       'touchend',"
                    "       'keydown',"
                    "       'keyup',"
                    "   ];"

                    // A proxy object to intercept AudioContexts and
                    // add them to the array for tracking and resuming later
                    "   self.AudioContext = new Proxy(self.AudioContext, {"
                    "       construct(target, args) {"
                    "           const result = new target(...args);"
                    "           audioContextList.push(result);"
                    "           return result;"
                    "       },"
                    "   });"

                    // To resume all AudioContexts being tracked
                    "   function resumeAllContexts(_event) {"
                    "       let count = 0;"

                    "       audioContextList.forEach(function(context) {"
                    "           if (context.state !== 'running') {"
                    "               context.resume();"
                    "           } else {"
                    "               count++;"
                    "           }"
                    "       });"

                    // If all the AudioContexts have now resumed then we unbind all
                    // the event listeners from the page to prevent unnecessary resume attempts
                    // Checking count !== 0 ensures that the user interaction happens AFTER the game started up
                    "       if (count !== 0) {"
                    "           if (count === audioContextList.length) {"
                    "               userInputEventNames.forEach(function(eventName) {"
                    "                   document.removeEventListener(eventName, resumeAllContexts);"
                    "               });"
                    "           }"
                    "       }"
                    "   }"

                    // We bind the resume function for each user interaction
                    // event on the page
                    "   userInputEventNames.forEach(function(eventName) {"
                    "       document.addEventListener(eventName, resumeAllContexts);"
                    "   });"
                    "})();"
                </script>
            </head>
            <body>
                <canvas id="bevy_canvas"></canvas>
                <script
                    type="module"
                    id="bevy_game"
                    data-game=(asset!(
                        "../../../assets/game/pkg/game_client.js", rename : "game"
                    ))
                    data-wasm=(asset!(
                        "../../../assets/game/pkg/game_client_bg.wasm", rename :
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
