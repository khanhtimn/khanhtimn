use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    platform::collections::HashMap,
    prelude::*,
};

use super::manifest::{CharacterManifestAsset, LoadedSpriteSheet};

#[derive(Default, TypePath)]
pub struct CharacterManifestLoader;

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
