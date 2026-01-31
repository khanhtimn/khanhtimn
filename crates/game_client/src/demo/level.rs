//! Level rendering for the game client.
//!
//! Renders the ground platform and syncs player transforms with
//! replicated positions from the server.

use bevy::prelude::*;
use khanhtimn_dev_common::{
    components::{Player, PlayerPosition},
    protocol::{GROUND_LEVEL, GROUND_WIDTH},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ground)
        .add_systems(Update, sync_player_transforms);
}

/// Spawn the ground platform mesh.
fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Ground"),
        Mesh2d(meshes.add(Rectangle::new(GROUND_WIDTH, 10.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.8, 0.4))),
        Transform::from_translation(Vec3::new(0.0, GROUND_LEVEL, 0.0)),
    ));
}

/// Sync player Transform with replicated PlayerPosition.
fn sync_player_transforms(mut query: Query<(&PlayerPosition, &mut Transform), With<Player>>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}
