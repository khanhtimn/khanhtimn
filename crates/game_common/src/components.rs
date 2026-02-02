//! Replicated components that sync between client and server.
//!
//! These components are registered with bevy_replicon and automatically
//! synchronized from the server to all connected clients.

use bevy::prelude::*;
use bevy_color::{Hsla, Srgba};
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

/// Player's visual color for identification.
///
/// Uses Bevy's Srgba for color representation.
/// Assigned by the server when a player connects.
#[derive(Component, Serialize, Deserialize, Default, Clone)]
pub struct PlayerColor(pub Srgba);

impl PlayerColor {
    /// Create from HSL values (hue: 0-360, saturation: 0-1, lightness: 0-1).
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self(Srgba::from(Hsla::new(h, s, l, 1.0)))
    }
}

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
