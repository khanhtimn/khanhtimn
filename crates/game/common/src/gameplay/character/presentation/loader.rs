use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    platform::collections::HashMap,
    prelude::*,
};

use super::manifest::{CharacterManifestAsset, LoadedSpriteSheet};

#[derive(Default, TypePath)]
pub struct CharacterManifestLoader;

// TODO: Major Asset Loading System Enhancements
// 1. Multi-Texture Atlas Packing: Support dynamically generated composite texture atlases (packing multiple individual clip sheets into single GPU texture maps to minimize render draw calls and state switches).
// 2. Asynchronous Texture Streaming & Deferred Loading: Support loading lightweight low-res proxies or core clips (idle/walk) first, lazily streaming heavy move/special-effect sprite sheets on demand.
// 3. Binary Serialization & Fast Parsing: Support binary RON / bincode or flatbuffers loaders for fast production startup times and reduced parsing overhead on WASM / mobile.
// 4. Hot-Reloading Dependency Re-indexing: Automatically detect changes to underlying `.png` sprite sheet files and trigger partial atlas re-generation without restarting the scene.
// 5. Custom Asset Loader Settings: Expose `type Settings = CharacterLoaderSettings` to configure sampler filtering (Nearest vs Linear) and mipmapping options per character manifest.

impl AssetLoader for CharacterManifestLoader {
    type Asset = CharacterManifestAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut manifest: CharacterManifestAsset = ron::de::from_bytes(&bytes)?;
        manifest.build_lookup_cache();

        let mut loaded_sheets = HashMap::default();

        for (sheet_key, sheet_def) in &manifest.sheets {
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(sheet_def.tile_width, sheet_def.tile_height),
                sheet_def.columns,
                sheet_def.rows,
                sheet_def.padding,
                sheet_def.offset,
            );

            let label = format!("atlas_layout_{}", sheet_key);
            let layout_handle = load_context.add_labeled_asset(label, layout);

            // Dynamically load referenced sprite sheet image dependency
            let image_handle: Handle<Image> = load_context.load(&sheet_def.image);

            loaded_sheets.insert(
                sheet_key.clone(),
                LoadedSpriteSheet {
                    image_handle,
                    atlas_layout_handle: layout_handle,
                },
            );
        }

        manifest.loaded_sheets = loaded_sheets;

        Ok(manifest)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
