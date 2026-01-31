//! Server-side input processing and physics.
//!
//! This module handles network input from clients and uses the
//! shared physics systems from game_common.

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use khanhtimn_dev_common::{
    PlayerJumpInput, PlayerMovementInput,
    components::{PlayerOwner, PlayerState},
    physics::SharedPhysicsPlugin,
    protocol::{JUMP_VELOCITY, MOVE_SPEED},
};

pub fn plugin(app: &mut App) {
    // Add shared physics systems
    app.add_plugins(SharedPhysicsPlugin);

    // Two observers - matching singleplayer pattern
    app.add_observer(apply_movement);
    app.add_observer(apply_jump);
}

/// Apply movement input from client.
fn apply_movement(
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

/// Apply jump input from client.
fn apply_jump(
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
