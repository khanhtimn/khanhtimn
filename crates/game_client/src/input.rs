//! Client input handling for multiplayer mode.
//!
//! Uses bevy_enhanced_input for action-based input and sends
//! separate events to the server for movement and jump.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_replicon::prelude::*;
use khanhtimn_dev_common::{Jump, Movement, PlayerJumpInput, PlayerMovementInput};

use crate::screens::{GameMode, GameScreen};

pub fn plugin(app: &mut App) {
    app.add_input_context::<MultiplayerPlayer>()
        .add_systems(
            OnEnter(GameScreen::Playing),
            setup_multiplayer_input.run_if(resource_equals(GameMode::Multiplayer)),
        )
        .add_observer(send_movement_to_server)
        .add_observer(send_jump_to_server);
}

/// Marker for multiplayer input entity.
#[derive(Component)]
pub struct MultiplayerPlayer;

/// Spawn input capture entity for multiplayer mode.
fn setup_multiplayer_input(mut commands: Commands) {
    bevy::log::info!("[Input] Setting up multiplayer input");

    commands.spawn((
        MultiplayerPlayer,
        actions!(MultiplayerPlayer[
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
                bindings![KeyCode::Space, KeyCode::KeyW, GamepadButton::South],
            )
        ]),
    ));
}

/// Send movement input to server.
fn send_movement_to_server(
    movement: On<Fire<Movement>>,
    game_mode: Res<GameMode>,
    mut commands: Commands,
    query: Query<Entity, With<MultiplayerPlayer>>,
) {
    if *game_mode != GameMode::Multiplayer {
        return;
    }

    if query.get(movement.context).is_ok() {
        commands.client_trigger(PlayerMovementInput {
            movement: movement.value,
        });
    }
}

/// Send jump input to server.
fn send_jump_to_server(
    jump: On<Fire<Jump>>,
    game_mode: Res<GameMode>,
    mut commands: Commands,
    query: Query<Entity, With<MultiplayerPlayer>>,
) {
    if *game_mode != GameMode::Multiplayer {
        return;
    }

    if query.get(jump.context).is_ok() {
        commands.client_trigger(PlayerJumpInput);
    }
}
