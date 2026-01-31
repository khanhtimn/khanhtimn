//! Main menu UI for mode selection.
//!
//! Provides buttons to choose between single player and multiplayer modes.

use bevy::prelude::*;

use crate::{
    screens::{GameMode, GameScreen},
    theme::prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameScreen::MainMenu), spawn_menu)
        .add_systems(OnExit(GameScreen::MainMenu), despawn_menu);
}

/// Marker component for the main menu root.
#[derive(Component)]
struct MainMenuRoot;

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        MainMenuRoot,
        widget::ui_root("Main Menu"),
        children![
            widget::header("Game Client"),
            widget::label("Select Mode"),
            widget::button("Single Player", on_single_player_click),
            widget::button("Multiplayer", on_multiplayer_click),
        ],
    ));
}

fn despawn_menu(mut commands: Commands, menu: Query<Entity, With<MainMenuRoot>>) {
    for entity in &menu {
        commands.entity(entity).despawn();
    }
}

fn on_single_player_click(
    _click: On<Pointer<Click>>,
    mut game_mode: ResMut<GameMode>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    bevy::log::info!("Starting Single Player mode");
    *game_mode = GameMode::SinglePlayer;
    next_screen.set(GameScreen::Playing);
}

fn on_multiplayer_click(
    _click: On<Pointer<Click>>,
    mut game_mode: ResMut<GameMode>,
    mut next_screen: ResMut<NextState<GameScreen>>,
) {
    bevy::log::info!("Starting Multiplayer mode");
    *game_mode = GameMode::Multiplayer;
    next_screen.set(GameScreen::Connecting);
}
