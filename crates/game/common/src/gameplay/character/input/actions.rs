use bevy::{ecs::component::Component};
use bevy_enhanced_input::prelude::InputAction;

#[derive(Component, Debug, Default)]
pub struct CharacterInput;

#[derive(InputAction)]
#[action_output(f32)]
pub struct Move;

#[derive(InputAction)]
#[action_output(f32)]
pub struct Jump;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dash;