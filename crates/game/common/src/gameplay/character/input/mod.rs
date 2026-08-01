use bevy::app::{App, FixedUpdate, Plugin};
use bevy_enhanced_input::{EnhancedInputPlugin, context::InputContextAppExt};

pub mod actions;
pub mod bindings;
pub mod context;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<actions::CharacterInput>()
            .add_systems(FixedUpdate, context::apply_input);
    }
}