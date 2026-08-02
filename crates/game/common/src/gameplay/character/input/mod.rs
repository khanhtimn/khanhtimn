use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::component::Component,
};
use bevy_enhanced_input::{EnhancedInputPlugin, context::InputContextAppExt};

pub mod actions;
mod systems;

#[derive(Component, Debug, Default)]
pub struct CharacterInput;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<CharacterInput>()
            .add_systems(FixedUpdate, systems::process_character_input);
    }
}
