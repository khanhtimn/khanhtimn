//! Game simulation plugin.
//!
//! Bundles all game logic: physics, spawning, and input handling.
//! Used by both server and single-player client.

use bevy::prelude::*;

use crate::{
    PlayerJumpInput, PlayerMovementInput,
    components::PlayerState,
    physics::SharedPhysicsPlugin,
    protocol::{JUMP_VELOCITY, MOVE_SPEED},
    spawning::{LocalPlayer, SpawningPlugin},
};

#[cfg(feature = "server")]
use crate::components::PlayerOwner;

#[cfg(feature = "server")]
use bevy_replicon::prelude::*;

/// Complete game simulation plugin.
///
/// Includes physics, spawning, and input handling.
/// Add this to run game logic locally.
pub struct GameSimulationPlugin;

impl Plugin for GameSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SharedPhysicsPlugin, SpawningPlugin));

        // Local input observers (single player)
        app.add_observer(apply_local_movement)
            .add_observer(apply_local_jump);

        // Server-only: network input observers
        #[cfg(feature = "server")]
        {
            app.add_observer(apply_network_movement)
                .add_observer(apply_network_jump);
        }
    }
}

// --- Local input (single player) ---

fn apply_local_movement(
    movement: On<PlayerMovementInput>,
    mut query: Query<&mut PlayerState, With<LocalPlayer>>,
) {
    if let Some(mut state) = query.iter_mut().next() {
        state.velocity.x = movement.movement * MOVE_SPEED;
    }
}

fn apply_local_jump(
    _jump: On<PlayerJumpInput>,
    mut query: Query<&mut PlayerState, With<LocalPlayer>>,
) {
    if let Some(mut state) = query.iter_mut().next()
        && state.is_grounded
    {
        state.velocity.y = JUMP_VELOCITY;
        state.is_grounded = false;
    }
}

// --- Network input (server) ---

#[cfg(feature = "server")]
fn apply_network_movement(
    trigger: On<FromClient<PlayerMovementInput>>,
    mut players: Query<(&PlayerOwner, &mut PlayerState)>,
) {
    let input = &trigger.message;

    for (owner, mut state) in &mut players {
        if **owner == trigger.client_id {
            state.velocity.x = input.movement * MOVE_SPEED;
            break;
        }
    }
}

#[cfg(feature = "server")]
fn apply_network_jump(
    trigger: On<FromClient<PlayerJumpInput>>,
    mut players: Query<(&PlayerOwner, &mut PlayerState)>,
) {
    for (owner, mut state) in &mut players {
        if **owner == trigger.client_id {
            if state.is_grounded {
                state.velocity.y = JUMP_VELOCITY;
                state.is_grounded = false;
                debug!("Player {:?} jumped", trigger.client_id);
            }
            break;
        }
    }
}
