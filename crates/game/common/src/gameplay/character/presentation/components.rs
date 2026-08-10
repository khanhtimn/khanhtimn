use bevy::prelude::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::manifest::CharacterManifestAsset;

bitflags! {
    /// Bitflags representing character animation playback and sprite rendering state.
    #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    #[serde(transparent)]
    pub struct AnimationPlaybackFlags: u8 {
        const NONE        = 0b0000_0000;
        const IS_FINISHED = 0b0000_0001;
        const IS_PAUSED   = 0b0000_0010;
        const FLIP_X      = 0b0000_0100;
        const FLIP_Y      = 0b0000_1000;
    }
}

/// Network-replicated presentation state for character animations.
/// Replicated via `bevy_replicon` (server-authoritative).
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct CharacterAnimationState {
    /// Active clip index within the character's manifest (0..N).
    pub clip_index: u16,

    /// Zero-based index of the current frame inside the active clip.
    pub frame_index: u16,

    /// Fixed 60Hz ticks elapsed within the current frame step.
    pub elapsed_ticks: u16,

    /// Hitstop/freeze counter (in ticks). Pauses animation progression when > 0.
    pub hitstop_ticks: u16,

    /// High-level playback status and sprite rendering flags.
    pub flags: AnimationPlaybackFlags,
}

impl CharacterAnimationState {
    pub fn is_finished(&self) -> bool {
        self.flags.contains(AnimationPlaybackFlags::IS_FINISHED)
    }

    pub fn set_finished(&mut self, finished: bool) {
        self.flags
            .set(AnimationPlaybackFlags::IS_FINISHED, finished);
    }

    pub fn is_paused(&self) -> bool {
        self.flags.contains(AnimationPlaybackFlags::IS_PAUSED)
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.flags.set(AnimationPlaybackFlags::IS_PAUSED, paused);
    }
}

/// Handle pointing to the loaded `CharacterManifestAsset`.
#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct CharacterManifestHandle(pub Handle<CharacterManifestAsset>);

/// User video presentation settings resource for sub-frame animation smoothing.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationVideoSettings {
    pub interpolation_mode: InterpolationMode,
    pub default_sprite_scale: f32,
}

impl Default for AnimationVideoSettings {
    fn default() -> Self {
        Self {
            interpolation_mode: InterpolationMode::Discrete60Hz,
            default_sprite_scale: 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InterpolationMode {
    #[default]
    Discrete60Hz,
    TransformInterpolated,
    ShaderCrossfade,
}
