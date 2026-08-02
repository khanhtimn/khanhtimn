use bevy::app::{App, Plugin};

use crate::gameplay::character::CharacterPlugin;

pub mod arena;
pub mod character;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterPlugin);
    }
}
