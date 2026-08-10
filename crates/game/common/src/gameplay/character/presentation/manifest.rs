use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LoopMode {
    #[default]
    Repeat,
    Once,
    HoldLast,
    PingPong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetDef {
    pub image: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
    #[serde(default)]
    pub padding: Option<UVec2>,
    #[serde(default)]
    pub offset: Option<UVec2>,
    // TODO: Add support for normal maps / emission maps for dynamic 2D lighting per sprite sheet
    // TODO: Add support for multi-resolution asset variants (SD vs HD texture paths)
}

#[derive(Debug, Clone)]
pub struct LoadedSpriteSheet {
    pub image_handle: Handle<Image>,
    pub atlas_layout_handle: Handle<TextureAtlasLayout>,
    // TODO: Store optional normal map & emission texture handles for dynamic 2D lighting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    /// TextureAtlas index for this frame
    pub sprite_index: usize,

    /// Duration of this frame step in fixed 60Hz ticks (e.g. 3 ticks = ~0.05s)
    pub duration_ticks: u16,

    /// Anchor/pivot offset (normalized or pixel coordinates)
    pub pivot: Option<Vec2>,
    // TODO: Hitboxes & Hurtboxes
    // - Add per-frame combat hurtbox shapes (AABB / capsule colliders for taking damage)
    // - Add per-frame combat hitbox shapes (active hit volumes with damage, hitstun, knockback vector)

    // TODO: Cancel Windows & Action Triggers
    // - Add `cancel_window`: Option<CancelWindowDef> allowing action cancels into jump, dash, or special moves on specific ticks
    // - Add `invulnerability_type`: Option<InvulnerabilityType> (e.g. Full, Strike, Grab, Upper-body)

    // TODO: Audio & Visual Feedback Triggers
    // - Add `sfx_event`: Option<String> / `sfx_handle` to play step/swing sound effects on keyframe ticks
    // - Add `vfx_spawns`: Vec<VfxSpawnDef> (e.g. dust particles on footstep or impact flash on swing)
    // - Add `camera_shake`: Option<CameraShakeImpulse> to trigger camera impact rumble on heavy landing/impact frames
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnimationClip {
    pub name: String,
    #[serde(default)]
    pub sheet: String,
    pub loop_mode: LoopMode,
    pub frames: Vec<AnimationFrame>,
    // TODO: Animation Blending & Layering
    // - Add `layer`: AnimationLayer (UpperBody, LowerBody, FullBody) for skeletal/partially-masked sprite blending
    // - Add `blend_weight`: f32 for crossfading between state transitions
    // - Add `root_motion`: Option<Vec2> for frame-driven physical displacement instead of pure velocity simulation
    // - Add `interruptible`: bool to indicate if animation can be interrupted by hitstun or movement inputs
}

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct CharacterManifestAsset {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub sheets: HashMap<String, SpriteSheetDef>,
    pub clips: Vec<CharacterAnimationClip>,

    /// Lookup cache mapping string clip names ("idle", "walk", "jump_up") to u16 clip indices
    #[serde(skip)]
    pub clip_name_to_index: HashMap<String, u16>,
    /// Map of dynamically resolved loaded sprite sheet handles keyed by sheet identifier
    #[serde(skip)]
    pub loaded_sheets: HashMap<String, LoadedSpriteSheet>,
    // TODO: Asset Loading & Hot-Reloading Improvements
    // - Support binary RON or bincode format serialization for optimized production builds
    // - Support hot-reloading cache invalidation when sprite sheets or manifests are modified on disk
    // - Add manifest schema validation for missing clip references or invalid sprite index bounds
}

impl CharacterManifestAsset {
    pub fn build_lookup_cache(&mut self) {
        self.clip_name_to_index.clear();
        for (idx, clip) in self.clips.iter().enumerate() {
            let clip_idx = u16::try_from(idx)
                .expect("Character manifest clip index exceeded maximum supported limit");
            self.clip_name_to_index.insert(clip.name.clone(), clip_idx);
        }
    }

    pub fn get_clip_index(&self, name: &str) -> Option<u16> {
        self.clip_name_to_index.get(name).copied()
    }
}
