use bevy::{asset::AssetApp, prelude::*};

pub mod components;
pub mod loader;
pub mod manifest;
pub mod systems;

pub use components::{
    AnimationPlaybackFlags, AnimationVideoSettings, CharacterAnimationState,
    CharacterManifestHandle, InterpolationMode,
};
pub use loader::CharacterManifestLoader;
pub use manifest::{
    AnimationFrame, CharacterAnimationClip, CharacterManifestAsset, LoadedSpriteSheet, LoopMode,
    SpriteSheetDef,
};
pub use systems::{
    advance_character_animations, update_character_animation_state, update_character_sprites,
};

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
