//! Single player mode logic.
//!
//! Handles local player spawning and input for single player mode.
//! Uses bevy_enhanced_input with observer pattern like the platformer example.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use khanhtimn_dev_common::{
    Jump, Movement,
    components::{Player, PlayerColor, PlayerPosition, PlayerState, Rgba},
    physics::SharedPhysicsPlugin,
    protocol::{GROUND_LEVEL, JUMP_VELOCITY, MOVE_SPEED, PLAYER_SIZE},
};
use rand::Rng;

use crate::screens::{GameMode, GameScreen};

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<LocalPlayer>()
        .add_systems(
            OnEnter(GameScreen::Playing),
            spawn_local_player.run_if(resource_equals(GameMode::SinglePlayer)),
        )
        // Input observers - apply movement/jump directly to PlayerState
        .add_observer(apply_movement)
        .add_observer(apply_jump);

    // Add shared physics plugin for single player
    app.add_plugins(SharedPhysicsPlugin);
}

/// Marker for the local player entity in single player mode.
#[derive(Component)]
pub struct LocalPlayer;

fn spawn_local_player(mut commands: Commands) {
    bevy::log::info!("[SinglePlayer] Spawning local player");

    // Generate random color
    let mut rng = rand::rng();
    let hue = rng.random_range(0.0..360.0);
    let color = Rgba::from_hsl(hue, 0.8, 0.6);

    // Spawn position: above ground
    let spawn_y = GROUND_LEVEL + PLAYER_SIZE.y / 2.0 + 100.0;
    let spawn_x = 0.0;

    commands.spawn((
        LocalPlayer,
        Player,
        PlayerPosition(Vec2::new(spawn_x, spawn_y)),
        PlayerState {
            velocity: Vec2::ZERO,
            is_grounded: false,
        },
        PlayerColor(color),
        // Input actions using bevy_enhanced_input
        // Scale applies MOVE_SPEED directly to the movement value
        actions!(LocalPlayer[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(MOVE_SPEED),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::KeyA, KeyCode::KeyD),
                    Bidirectional::new(KeyCode::ArrowLeft, KeyCode::ArrowRight),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, GamepadButton::South],
            )
        ]),
    ));

    bevy::log::info!(
        "[SinglePlayer] Local player spawned at ({}, {})",
        spawn_x,
        spawn_y
    );
}

/// Apply movement input directly to velocity.
/// Scale already applies MOVE_SPEED, so we set velocity.x directly.
fn apply_movement(
    movement: On<Fire<Movement>>,
    game_mode: Res<GameMode>,
    mut query: Query<&mut PlayerState>,
) {
    if *game_mode != GameMode::SinglePlayer {
        return;
    }

    if let Ok(mut state) = query.get_mut(movement.context) {
        state.velocity.x = movement.value;
    }
}

/// Apply jump input - set vertical velocity if grounded.
fn apply_jump(jump: On<Fire<Jump>>, game_mode: Res<GameMode>, mut query: Query<&mut PlayerState>) {
    if *game_mode != GameMode::SinglePlayer {
        return;
    }

    if let Ok(mut state) = query.get_mut(jump.context) {
        if state.is_grounded {
            state.velocity.y = JUMP_VELOCITY;
            state.is_grounded = false;
        }
    }
}
