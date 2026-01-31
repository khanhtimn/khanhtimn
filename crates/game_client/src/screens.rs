//! Game screen states and mode management.
//!
//! Defines the game's state machine for navigation between
//! menu, single player, and multiplayer modes.

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<GameScreen>().init_resource::<GameMode>();
}

/// The game's screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameScreen {
    /// Main menu - mode selection.
    #[default]
    MainMenu,
    /// Connecting to the game server (multiplayer only).
    Connecting,
    /// Main gameplay - playing the game.
    Playing,
    /// Disconnected from server (multiplayer only).
    Disconnected,
}

/// The selected game mode.
///
/// Determines whether physics runs locally (single player)
/// or on the server (multiplayer).
#[derive(Resource, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum GameMode {
    #[default]
    SinglePlayer,
    Multiplayer,
}
