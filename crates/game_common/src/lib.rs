//! Shared game logic for client and server.
//!
//! This crate contains all replicated components, network events,
//! and shared constants used by both the WASM client and native server.

use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub mod components;
pub mod events;
pub mod input;
pub mod physics;
pub mod protocol;

pub use input::{Jump, Movement, PlayerJumpInput, PlayerMovementInput};
pub use physics::SharedPhysicsPlugin;

/// Plugin that registers all shared game logic.
///
/// This should be added to both client and server apps.
pub struct CommonGamePlugin;

impl Plugin for CommonGamePlugin {
    fn build(&self, app: &mut App) {
        // Register replicated components
        app.replicate::<components::PlayerPosition>()
            .replicate::<components::PlayerState>()
            .replicate::<components::PlayerColor>()
            .replicate::<components::Player>();

        // Register client->server events (separate for movement and jump)
        app.add_client_event::<PlayerMovementInput>(Channel::Ordered)
            .add_client_event::<PlayerJumpInput>(Channel::Ordered);
    }
}
