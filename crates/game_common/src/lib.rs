//! Shared game logic for client and server.
//!
//! This crate contains all replicated components, network events,
//! and shared constants used by both the WASM client and native server.

use bevy::prelude::*;

// Re-export for shared types/traits used across crates
// (ClientTriggerExt, InputAction, etc.)
pub use bevy_enhanced_input;
pub use bevy_replicon;
use bevy_replicon::prelude::*;

pub mod components;
pub mod input;
pub mod physics;
pub mod protocol;
pub mod simulation;
pub mod spawning;

pub use input::{Jump, Movement, PlayerJumpInput, PlayerMovementInput};
pub use physics::SharedPhysicsPlugin;
pub use simulation::GameSimulationPlugin;
pub use spawning::{LocalPlayer, SpawnLocalPlayer, SpawningPlugin};

/// Plugin that registers all shared game logic.
///
/// This should be added to both client and server apps.
/// For game simulation, also add `GameSimulationPlugin`.
pub struct CommonGamePlugin;

impl Plugin for CommonGamePlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<components::PlayerPosition>()
            .replicate::<components::PlayerState>()
            .replicate::<components::PlayerColor>()
            .replicate::<components::Player>();

        app.add_client_event::<PlayerMovementInput>(Channel::Ordered)
            .add_client_event::<PlayerJumpInput>(Channel::Ordered);
    }
}
