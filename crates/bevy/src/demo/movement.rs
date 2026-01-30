//! Handle player input and translate it into movement through a physics-based
//! character controller.
//!
//! The character controller uses:
//! - Observer-based input handling via `bevy_enhanced_input`
//! - Physics simulation with gravity and ground collision
//! - Horizontal movement and jump mechanics

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use super::input::{Jump, Movement};
use crate::{AppSystems, PausableSystems};

/// Physics constants for the character controller.
pub const GROUND_LEVEL: f32 = -200.0;
pub const GROUND_WIDTH: f32 = 1280.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 100.0);
pub const JUMP_VELOCITY: f32 = 500.0;
pub const GRAVITY: f32 = 1200.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (calculate_physics, clamp_to_bounds)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_observer(apply_movement);
    app.add_observer(apply_jump);
}

/// Physics component for entities with velocity and grounded state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Physics {
    pub velocity: Vec2,
    pub is_grounded: bool,
}

/// Observer that applies horizontal movement from input.
fn apply_movement(movement: On<Fire<Movement>>, mut query: Query<&mut Physics>) {
    if let Ok(mut physics) = query.get_mut(movement.context) {
        physics.velocity.x = movement.value;
    }
}

/// Observer that applies jump when grounded.
fn apply_jump(jump: On<Fire<Jump>>, mut query: Query<&mut Physics>) {
    if let Ok(mut physics) = query.get_mut(jump.context) {
        if physics.is_grounded {
            physics.velocity.y = JUMP_VELOCITY;
            physics.is_grounded = false;
        }
    }
}

/// System that applies physics simulation (gravity, velocity, ground collision).
fn calculate_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut Physics)>) {
    for (mut transform, mut physics) in query.iter_mut() {
        // Apply gravity
        physics.velocity.y -= GRAVITY * time.delta_secs();

        // Apply velocity
        transform.translation.y += physics.velocity.y * time.delta_secs();
        transform.translation.x += physics.velocity.x * time.delta_secs();

        // Check for ground collision
        let grounded_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0;
        if transform.translation.y <= grounded_y {
            transform.translation.y = grounded_y;
            physics.velocity.y = 0.0;
            physics.is_grounded = true;
        }
    }
}

/// System that clamps entities within horizontal bounds.
fn clamp_to_bounds(mut query: Query<&mut Transform, With<Physics>>) {
    let max_x = GROUND_WIDTH / 2.0 - PLAYER_SIZE.x / 2.0;
    for mut transform in query.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    }
}
