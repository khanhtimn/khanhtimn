use bevy::{
    ecs::component::Component,
    platform::collections::HashMap,
    prelude::{Deref, DerefMut, Handle, Image},
    time::Timer,
};

/// Decouples visual frame rate parameters from game tick logic.
///
/// Allows animation speed (FPS) to be tuned independently per entity
/// or dynamically altered by status effects without mutating frame sequences.
#[derive(Component, Debug, Clone)]
pub struct AnimationConfig {
    pub fps: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self { fps: 12.5 }
    }
}

/// Internal timer driving keyframe transitions.
///
/// Ticked by Bevy's `Time` resource in the update schedule to signal when the visual
/// sprite should advance to the next frame.
#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);

/// Stores the zero-based index of the currently displayed animation keyframe.
///
/// Tracks progress through the active frame sequence. Separating index state
/// from the image handle container allows external systems to inspect or reset
/// animation progression cleanly.
#[derive(Component, Default, Deref, DerefMut, Debug)]
pub struct AnimationFrameIndex(pub usize);

/// Categorizes high-level character presentation states.
///
/// Defines the semantic visual actions a character can express.
/// Serves as the lookup key in [`CharacterAnimationMap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CharacterAnimationState {
    #[default]
    Idle,
    Walk,
    JumpUp,
    JumpDown,
    KnockDown,
}

/// Data container mapping character states to frame image handle sequences.
///
/// Acts as a lightweight presentation palette holding loaded asset handles
/// for each visual state.
///
/// *Architecture Note*: This is a naive POC structure for early prototyping.
/// In future revisions, this will be rearchitected to integrate with a formal state machine
/// or Bevy's `TextureAtlas` sprite sheets.
#[derive(Component, Default, Debug, Clone)]
pub struct CharacterAnimationMap {
    pub current_state: CharacterAnimationState,
    pub animations: HashMap<CharacterAnimationState, Vec<Handle<Image>>>,
}
