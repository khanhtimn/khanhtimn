//! Player spawning and despawning on client connect/disconnect.
//!
//! This module manages the lifecycle of player entities based on
//! client connection events.

use bevy::prelude::*;
use game_common::{
    bevy_replicon::prelude::*,
    components::{Player, PlayerColor, PlayerOwner, PlayerPosition, PlayerState, Rgba},
    protocol::{GROUND_LEVEL, PLAYER_SIZE},
};
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.add_observer(on_client_connect)
        .add_observer(on_client_disconnect);
}

/// Observer triggered when a new client connects.
fn on_client_connect(trigger: On<Add, ConnectedClient>, mut commands: Commands) {
    let client_entity = trigger.event().entity;
    info!("Client connected: {:?}", client_entity);

    // Generate random color for this player using HSL for better distribution
    let mut rng = rand::rng();
    let hue = rng.random_range(0.0..360.0);
    let color = Rgba::from_hsl(hue, 0.8, 0.6);

    // Spawn position: above ground with random horizontal offset
    let spawn_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0 + 100.0;
    let spawn_x = rng.random_range(-200.0..200.0);

    commands.spawn((
        Player,
        PlayerPosition(Vec2::new(spawn_x, spawn_y)),
        PlayerState {
            velocity: Vec2::ZERO,
            is_grounded: false,
        },
        PlayerColor(color),
        PlayerOwner(ClientId::Client(client_entity)),
    ));

    info!(
        "Spawned player for client {:?} at ({}, {})",
        client_entity, spawn_x, spawn_y
    );
}

/// Observer triggered when a client disconnects.
fn on_client_disconnect(
    trigger: On<Remove, ConnectedClient>,
    mut commands: Commands,
    players: Query<(Entity, &PlayerOwner)>,
) {
    let client_entity = trigger.event().entity;
    info!("Client disconnected: {:?}", client_entity);

    // Find and despawn the player entity belonging to this client
    for (entity, owner) in &players {
        if **owner == ClientId::Client(client_entity) {
            commands.entity(entity).despawn();
            info!("Despawned player entity for client {:?}", client_entity);
            break;
        }
    }
}
