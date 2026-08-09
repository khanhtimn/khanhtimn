pub mod gameplay;

use bevy::app::App;
use bevy::ecs::VariantDefaults;
use bevy::prelude::*;

pub mod prelude {
    pub use crate::GamePlugin;
    pub use crate::GameState;
    pub use crate::gameplay::{
        GameplayPlugin,
        character::{
            Character, CharacterInput, CharacterLocomotion, CharacterPlugin, actions,
            presentation::{
                AnimationConfig, AnimationFrameIndex, AnimationTimer, CharacterAnimationMap,
                CharacterAnimationState, PresentationPlugin,
            },
        },
    };
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, VariantDefaults)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
    GameEnd,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(gameplay::GameplayPlugin);
    }
}
