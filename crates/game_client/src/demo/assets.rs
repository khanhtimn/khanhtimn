//! Player sprite assets loading.
//!
//! Handles loading of player textures and audio assets.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayerAssets>();
}

/// Player's visual and audio assets.
#[derive(Resource)]
pub struct PlayerAssets {
    pub ducky: Handle<Image>,
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
