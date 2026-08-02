use bevy_enhanced_input::prelude::InputAction;

#[derive(InputAction)]
#[action_output(f32)]
pub struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Jump;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Crouch;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UpModifier;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dash;
