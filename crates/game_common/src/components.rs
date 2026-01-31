//! Replicated components that sync between client and server.
//!
//! These components are registered with bevy_replicon and automatically
//! synchronized from the server to all connected clients.

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

/// Player's position, replicated from server to clients.
///
/// The server is authoritative over this component - clients receive
/// updates but do not modify it directly.
#[derive(Component, Serialize, Deserialize, Default, Deref, DerefMut, Clone)]
pub struct PlayerPosition(pub Vec2);

/// Player's current physics state.
///
/// Contains velocity and grounded state. While replicated, clients
/// may use this for interpolation/prediction.
#[derive(Component, Serialize, Deserialize, Default, Clone)]
pub struct PlayerState {
    pub velocity: Vec2,
    pub is_grounded: bool,
}

/// Simple RGBA color representation for network serialization.
///
/// This is used instead of bevy's Color to avoid feature dependencies
/// in the common crate and ensure easy serialization.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create from HSL values (hue: 0-360, saturation: 0-1, lightness: 0-1).
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: r + m,
            g: g + m,
            b: b + m,
            a: 1.0,
        }
    }
}

/// Player's visual color for identification.
///
/// Assigned by the server when a player connects.
#[derive(Component, Serialize, Deserialize, Default, Clone)]
pub struct PlayerColor(pub Rgba);

/// Marker for the local player (client-only, not replicated).
///
/// Used by the client to identify which player entity belongs to this client.
#[derive(Component)]
pub struct LocalPlayer;

/// Ownership marker (server-only, not replicated).
///
/// Identifies which client owns this player entity.
#[derive(Component, Clone, Copy, Deref)]
pub struct PlayerOwner(pub ClientId);

/// Marker for replicated player entities.
///
/// This component requires PlayerPosition, PlayerState, PlayerColor,
/// and Replicated to be present on the entity.
#[derive(Component, Serialize, Deserialize, Default)]
#[require(PlayerPosition, PlayerState, PlayerColor, Replicated)]
pub struct Player;

