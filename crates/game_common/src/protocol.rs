//! Network protocol constants and shared physics values.
//!
//! These constants are used by both client and server to ensure
//! consistent behavior across the network.

use bevy::prelude::Vec2;

/// Protocol version for netcode authentication.
/// Used to ensure client and server are compatible.
pub const PROTOCOL_ID: u64 = 0x4B48414E48_544E; // "KHANH_TN" in hex

/// Default server port for WebTransport connections.
pub const DEFAULT_PORT: u16 = 4433;

/// Maximum number of clients the server will accept.
pub const MAX_CLIENTS: usize = 64;

// Physics constants - shared for consistent behavior and potential client-side prediction.

/// Y-coordinate of the ground plane.
pub const GROUND_LEVEL: f32 = -200.0;

/// Width of the playable area.
pub const GROUND_WIDTH: f32 = 1280.0;

/// Player collision box size.
pub const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 100.0);

/// Initial upward velocity when jumping.
pub const JUMP_VELOCITY: f32 = 300.0;

/// Acceleration due to gravity (pixels/second²).
pub const GRAVITY: f32 = 900.0;

/// Horizontal movement speed (pixels/second).
pub const MOVE_SPEED: f32 = 450.0;
