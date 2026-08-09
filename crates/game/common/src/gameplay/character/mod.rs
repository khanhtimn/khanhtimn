use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
};

pub mod combat;
pub mod constitution;
pub mod input;
pub mod locomotion;
pub mod presentation;

pub use input::{CharacterInput, actions};
pub use locomotion::CharacterLocomotion;

#[derive(Component, Debug, Default)]
#[require(CharacterLocomotion)]
pub struct Character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            locomotion::LocomotionPlugin,
            presentation::PresentationPlugin,
        ));
    }
}
