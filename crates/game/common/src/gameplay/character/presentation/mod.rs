use bevy::{asset::AssetApp, prelude::*};

pub mod components;
pub mod loader;
pub mod manifest;
pub mod systems;

pub use components::*;
pub use loader::*;
pub use manifest::*;
pub use systems::*;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterManifestAsset>()
            .init_asset_loader::<CharacterManifestLoader>()
            .init_resource::<AnimationVideoSettings>()
            .add_systems(
                FixedUpdate,
                (
                    update_character_animation_state,
                    advance_character_animations,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, update_character_sprites);
    }
}
