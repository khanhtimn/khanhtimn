//! Input handling for the game client.
//!
//! Captures keyboard input and emits player input events.
//! In single player mode, these events are handled locally.
//! In multiplayer mode, they are sent to the server via replicon.

use bevy::prelude::*;
use game_common::{
    PlayerJumpInput, PlayerMovementInput,
    bevy_enhanced_input::prelude::*,
    bevy_replicon::prelude::*,
    input::{Jump, Movement},
};

use crate::screens::{GameMode, GameScreen};

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin);

    app.add_input_context::<PlayerInputContext>();

    app.add_systems(OnEnter(GameScreen::Playing), setup_input);

    app.add_observer(on_movement_action);
    app.add_observer(on_movement_stop);
    app.add_observer(on_jump_action);
}

/// Input context component for player controls.
#[derive(Component)]
struct PlayerInputContext;

/// Entity that holds the input context.
#[derive(Component)]
pub struct InputController;

/// Set up input context when entering the Playing screen.
fn setup_input(mut commands: Commands, existing: Query<Entity, With<InputController>>) {
    // Don't spawn if already exists
    if !existing.is_empty() {
        return;
    }

    commands.spawn((
        Name::new("Input Controller"),
        InputController,
        PlayerInputContext,
        actions!(PlayerInputContext[
            (
                Action::<Movement>::new(),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![
                    KeyCode::Space,
                    KeyCode::KeyW,
                    KeyCode::ArrowUp,
                ],
            ),
        ]),
    ));
}

/// Handle movement action and emit PlayerMovementInput event.
fn on_movement_action(
    trigger: On<Fire<Movement>>,
    mut commands: Commands,
    game_mode: Res<GameMode>,
) {
    let movement = trigger.value;

    let input = PlayerMovementInput { movement };

    match *game_mode {
        GameMode::SinglePlayer => {
            commands.trigger(input);
        }
        GameMode::Multiplayer => {
            commands.client_trigger(input);
        }
    }
}

/// Handle movement action stopping (key released) - reset velocity to 0.
fn on_movement_stop(
    _trigger: On<Complete<Movement>>,
    mut commands: Commands,
    game_mode: Res<GameMode>,
) {
    let input = PlayerMovementInput { movement: 0.0 };

    match *game_mode {
        GameMode::SinglePlayer => {
            commands.trigger(input);
        }
        GameMode::Multiplayer => {
            commands.client_trigger(input);
        }
    }
}

/// Handle jump action and emit PlayerJumpInput event.
fn on_jump_action(_trigger: On<Start<Jump>>, mut commands: Commands, game_mode: Res<GameMode>) {
    let input = PlayerJumpInput;

    match *game_mode {
        GameMode::SinglePlayer => {
            // Local: trigger observer directly
            commands.trigger(input);
        }
        GameMode::Multiplayer => {
            // Network: send to server via replicon client event
            commands.client_trigger(input);
        }
    }
}
