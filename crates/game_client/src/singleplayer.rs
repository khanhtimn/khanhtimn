//! Single player mode logic.
//!
//! Uses GameSimulationPlugin for game logic.
//! Handles input via bevy_enhanced_input and fires events.

use bevy::prelude::*;
use game_common::{
    GameSimulationPlugin, Jump, LocalPlayer, Movement, PlayerJumpInput, PlayerMovementInput,
    SpawnLocalPlayer, bevy_enhanced_input::prelude::*, protocol::MOVE_SPEED,
};

use crate::screens::{GameMode, GameScreen};

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<LocalPlayer>()
        // Add game simulation (physics, spawning, input handling)
        .add_plugins(GameSimulationPlugin)
        // Spawn player and setup input on entering Playing state
        .add_systems(
            OnEnter(GameScreen::Playing),
            spawn_player.run_if(resource_equals(GameMode::SinglePlayer)),
        )
        .add_systems(
            Update,
            setup_input
                .run_if(resource_equals(GameMode::SinglePlayer).and(in_state(GameScreen::Playing))),
        )
        // Convert bevy_enhanced_input actions to game events
        .add_observer(on_movement_fire)
        .add_observer(on_jump_fire);
}

fn spawn_player(mut commands: Commands) {
    bevy::log::info!("[SinglePlayer] Requesting player spawn");
    commands.trigger(SpawnLocalPlayer);
}

fn setup_input(
    mut commands: Commands,
    player: Query<Entity, (With<LocalPlayer>, Without<Actions<LocalPlayer>>)>,
) {
    // Add input actions to player once spawned (if not already added)
    if let Some(entity) = player.iter().next() {
        bevy::log::info!("[SinglePlayer] Setting up input for player");
        commands.entity(entity).insert(actions!(LocalPlayer[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(1.0),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp, GamepadButton::South],
            )
        ]));
    }
}

/// Convert Movement action to PlayerMovementInput event.
fn on_movement_fire(
    movement: On<Fire<Movement>>,
    game_mode: Res<GameMode>,
    mut commands: Commands,
    query: Query<Entity, With<LocalPlayer>>,
) {
    if *game_mode != GameMode::SinglePlayer {
        return;
    }

    if query.get(movement.context).is_ok() {
        commands.trigger(PlayerMovementInput {
            movement: movement.value * MOVE_SPEED,
        });
    }
}

/// Convert Jump action to PlayerJumpInput event.
fn on_jump_fire(
    jump: On<Fire<Jump>>,
    game_mode: Res<GameMode>,
    mut commands: Commands,
    query: Query<Entity, With<LocalPlayer>>,
) {
    if *game_mode != GameMode::SinglePlayer {
        return;
    }

    if query.get(jump.context).is_ok() {
        commands.trigger(PlayerJumpInput);
    }
}
