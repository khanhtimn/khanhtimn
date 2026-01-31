//! Shared physics systems for both server and client.
//!
//! These systems implement deterministic physics simulation that can run
//! on the server (multiplayer) or client (single player).
//!
//! Designed for future `bevy_rewind` integration:
//! - All physics state stored in components
//! - Systems are deterministic (same inputs = same outputs)
//! - No external randomness or time-dependent behavior

use bevy::prelude::*;

use crate::{
    components::{Player, PlayerPosition, PlayerState},
    protocol::{GRAVITY, GROUND_LEVEL, GROUND_WIDTH, PLAYER_SIZE},
};

/// Plugin that adds shared physics systems.
///
/// Should be added to both server and client apps when physics
/// needs to run locally (server always, client in single player mode).
pub struct SharedPhysicsPlugin;

impl Plugin for SharedPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, calculate_physics);
    }
}

/// Physics simulation system.
///
/// Applies gravity, velocity, and ground collision.
/// This is deterministic: given the same state and delta time,
/// it will always produce the same result.
pub fn calculate_physics(
    time: Res<Time>,
    mut players: Query<(&mut PlayerPosition, &mut PlayerState), With<Player>>,
) {
    for (mut position, mut state) in &mut players {
        // Apply gravity
        state.velocity.y -= GRAVITY * time.delta_secs();

        // Apply velocity to position
        position.0.y += state.velocity.y * time.delta_secs();
        position.0.x += state.velocity.x * time.delta_secs();

        // Clamp horizontal position
        let max_x = GROUND_WIDTH / 2.0 - PLAYER_SIZE.x / 2.0;
        position.0.x = position.0.x.clamp(-max_x, max_x);

        // Ground collision detection
        let grounded_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0;
        if position.0.y <= grounded_y {
            position.0.y = grounded_y;
            state.velocity.y = 0.0;
            state.is_grounded = true;
        }
    }
}
