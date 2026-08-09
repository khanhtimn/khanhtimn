use bevy::app::{App, Plugin, Update};

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_character_frames);
    }
}
