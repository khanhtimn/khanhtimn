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
}

#[derive(Debug, Clone)]
pub struct LoadedSpriteSheet {
    pub image_handle: Handle<Image>,
    pub atlas_layout_handle: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    /// TextureAtlas index for this frame
    pub sprite_index: usize,

    /// Duration of this frame step in fixed 60Hz ticks (e.g. 3 ticks = ~0.05s)
    pub duration_ticks: u16,

    /// Anchor/pivot offset (normalized or pixel coordinates)
    pub pivot: Option<Vec2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnimationClip {
    pub name: String,
    #[serde(default)]
    pub sheet: String,
    pub loop_mode: LoopMode,
    pub frames: Vec<AnimationFrame>,
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
}

impl CharacterManifestAsset {
    pub fn build_lookup_cache(&mut self) {
        self.clip_name_to_index.clear();
        for (idx, clip) in self.clips.iter().enumerate() {
            self.clip_name_to_index
                .insert(clip.name.clone(), idx as u16);
        }
    }

    pub fn get_clip_index(&self, name: &str) -> Option<u16> {
        self.clip_name_to_index.get(name).copied()
    }
}
