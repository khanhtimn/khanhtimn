use std::path::PathBuf;

use topcoat::router::tower::TowerRoute;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Methods, Path as RoutePath},
};
use tower_http::services::ServeDir;

mod app;
mod components;
mod models;

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

    let router = app::router()
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
