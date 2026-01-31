//! Network events sent between client and server.
//!
//! These events are registered with bevy_replicon and transmitted
//! over the network via the renet2 transport.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Input event sent from client to server.
///
/// Contains the player's desired movement direction and jump state.
/// The server receives this and applies it to the player's physics.
#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInput {
    /// Normalized movement direction (-1.0 to 1.0 on each axis).
    pub movement: Vec2,
    /// Whether the player wants to jump this frame.
    pub jump: bool,
}
