use std::{
    path::{Component, Path, PathBuf},
    sync::LazyLock,
};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    router::{
        Body, Response, Router, RouterBuilderDiscoverExt, StatusCode, page, raw_path_params, route,
    },
    tailwind,
    view::view,
};

static WORKSPACE_ASSETS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("assets"))
        .unwrap_or_else(|| PathBuf::from("assets"))
});

#[route(GET "/assets/{*file_path}")]
async fn serve_game_assets(cx: &Cx) -> Result<Response> {
    let raw_params = raw_path_params(cx);
    let rel_path_str = raw_params
        .iter()
        .find(|(k, _)| *k == "file_path")
        .map(|(_, v)| v)
        .unwrap_or("");

    let rel_path = Path::new(rel_path_str);

    for component in rel_path.components() {
        if matches!(component, Component::ParentDir) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid asset path"))?);
        }
    }

    let full_path = WORKSPACE_ASSETS_DIR.join(rel_path);

    let canonical_assets = match tokio::fs::canonicalize(&*WORKSPACE_ASSETS_DIR).await {
        Ok(path) => path,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Asset Not Found"))?);
        }
    };

    let canonical_file = match tokio::fs::canonicalize(&full_path).await {
        Ok(path) => path,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Asset Not Found"))?);
        }
    };

    if !canonical_file.starts_with(&canonical_assets) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Access Denied"))?);
    }

    match tokio::fs::read(&canonical_file).await {
        Ok(bytes) => {
            let mut builder = Response::builder().header(
                "content-type",
                mime_guess::from_path(&canonical_file)
                    .first_or_octet_stream()
                    .as_ref(),
            );

            if cfg!(debug_assertions) {
                builder = builder
                    .header("cache-control", "no-cache, no-store, must-revalidate")
                    .header("pragma", "no-cache");
            } else {
                builder = builder.header("cache-control", "public, max-age=3600");
            }

            Ok(builder.body(Body::from(bytes))?)
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Asset Not Found"))?),
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Access Denied"))?),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))?),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(err) = run().await {
        eprintln!("================================================");
        eprintln!("CRITICAL ERROR ON STARTUP: {err:?}");
        eprintln!("Current Directory: {:?}", std::env::current_dir());
        eprintln!(
            "target/assets exists? {}",
            std::path::Path::new("target/assets").exists()
        );
        eprintln!("assets exists? {}", std::path::Path::new("assets").exists());
        eprintln!("================================================");
        return Err(err);
    }
    Ok(())
}

async fn run() -> Result<()> {
    println!("Starting personal-page server...");
    println!("Current Directory: {:?}", std::env::current_dir());
    println!("Loading AssetBundle from target/assets...");

    let assets = match AssetBundle::load_dir("target/assets") {
        Ok(bundle) => bundle,
        Err(err) => {
            eprintln!(
                "Warning: Could not load AssetBundle from target/assets ({err:?}). Falling back to empty AssetBundle."
            );
            AssetBundle::default()
        }
    };
    let router = Router::builder().discover().assets(assets).build();

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
