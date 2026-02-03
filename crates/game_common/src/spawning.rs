//! Player spawning and despawning.
//!
//! Shared spawning logic used by both server (on client connect)
//! and single-player client (on local spawn event).

use bevy::log::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{Player, PlayerColor, PlayerPosition, PlayerState},
    protocol::{GROUND_LEVEL, PLAYER_SIZE},
};

#[cfg(feature = "server")]
use crate::components::PlayerOwner;

#[cfg(feature = "server")]
use bevy_replicon::prelude::*;

/// Event to spawn a local player (single player mode).
#[derive(Event)]
pub struct SpawnLocalPlayer;

/// Marker for the local player entity.
#[derive(Component)]
pub struct LocalPlayer;

/// Spawning plugin - registers spawning systems.
pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_local_player);

        // Server-only: spawn on client connect
        #[cfg(feature = "server")]
        {
            app.add_observer(on_client_connect)
                .add_observer(on_client_disconnect);
        }
    }
}

/// Spawn a local player (single player mode).
fn on_spawn_local_player(_trigger: On<SpawnLocalPlayer>, mut commands: Commands) {
    info!("[Spawning] Spawning local player");

    let mut rng = rand::rng();
    let hue = rng.random_range(0.0..360.0);
    let color = PlayerColor::from_hsl(hue, 0.8, 0.6);

    let spawn_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0 + 100.0;
    let spawn_x = 0.0;

    commands.spawn((
        Player,
        PlayerPosition(Vec2::new(spawn_x, spawn_y)),
        PlayerState {
            velocity: Vec2::ZERO,
            is_grounded: false,
        },
        color,
        LocalPlayer,
    ));

    info!(
        "[Spawning] Local player spawned at ({}, {})",
        spawn_x, spawn_y
    );
}

// --- Server-only spawning ---

#[cfg(feature = "server")]
fn on_client_connect(trigger: On<Add, ConnectedClient>, mut commands: Commands) {
    let client_entity = trigger.event().entity;
    info!("Client connected: {:?}", client_entity);

    let mut rng = rand::rng();
    let hue = rng.random_range(0.0..360.0);
    let color = PlayerColor::from_hsl(hue, 0.8, 0.6);

    let spawn_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0 + 100.0;
    let spawn_x = rng.random_range(-200.0..200.0);

    commands.spawn((
        Player,
        PlayerPosition(Vec2::new(spawn_x, spawn_y)),
        PlayerState {
            velocity: Vec2::ZERO,
            is_grounded: false,
        },
        color,
        PlayerOwner(ClientId::Client(client_entity)),
    ));

    info!(
        "Spawned player for client {:?} at ({}, {})",
        client_entity, spawn_x, spawn_y
    );
}

#[cfg(feature = "server")]
fn on_client_disconnect(
    trigger: On<Remove, ConnectedClient>,
    mut commands: Commands,
    players: Query<(Entity, &PlayerOwner)>,
) {
    let client_entity = trigger.event().entity;
    info!("Client disconnected: {:?}", client_entity);

    for (entity, owner) in &players {
        if **owner == ClientId::Client(client_entity) {
            commands.entity(entity).despawn();
            info!("Despawned player entity for client {:?}", client_entity);
            break;
        }
    }
}
