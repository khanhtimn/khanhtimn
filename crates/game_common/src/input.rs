//! Shared input actions for both client and server.
//!
//! Uses `bevy_enhanced_input` for action-based input handling.
//! These actions can be used locally (single player) or sent over network (multiplayer).

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use serde::{Deserialize, Serialize};

/// Horizontal movement action.
/// Output is a normalized f32 value (-1.0 to 1.0).
#[derive(Debug, InputAction)]
#[action_output(f32)]
pub struct Movement;

/// Jump action.
/// Output is a bool indicating whether jump was triggered.
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Jump;

/// Movement input sent from client to server.
#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct PlayerMovementInput {
    /// Normalized horizontal movement (-1.0 to 1.0).
    pub movement: f32,
}

/// Jump input sent from client to server.
#[derive(Event, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerJumpInput;
