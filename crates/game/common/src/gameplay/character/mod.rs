use bevy::{app::{App, Plugin}, ecs::bundle::Bundle};

pub mod combat;
pub mod constitution;
pub mod input;
pub mod locomotion;
pub mod presentation;

#[derive(Bundle, Default)]
pub struct Character {
    pub input: input::actions::CharacterInput,
    pub locomotion: locomotion::Locomotion,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::InputPlugin);
    }
}