//! Player input handling using bevy_enhanced_input.
//!
//! This module defines input actions and their bindings for the player character.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<PlayerInput>();
}

/// Marker component for player input context.
/// This is attached to the player entity to enable input handling.
#[derive(Component)]
pub struct PlayerInput;

/// Horizontal movement action (left/right).
#[derive(Debug, InputAction)]
#[action_output(f32)]
pub struct Movement;

/// Jump action.
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Jump;

/// Creates the actions bundle for the player with all input bindings.
pub fn player_actions() -> impl Bundle {
    actions!(PlayerInput[
        (
            Action::<Movement>::new(),
            DeadZone::default(),
            SmoothNudge::default(),
            Scale::splat(450.0),
            Bindings::spawn((
                Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                Axial::left_stick(),
            )),
        ),
        (
            Action::<Jump>::new(),
            bindings![
                KeyCode::Space,
                KeyCode::KeyW,
                KeyCode::ArrowUp,
                GamepadButton::South
            ],
        )
    ])
}
