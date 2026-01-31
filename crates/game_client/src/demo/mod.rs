//! Demo gameplay module.
//!
//! This module contains the player visualization, animation, and asset loading
//! for the multiplayer game client. Physics and game logic run on the server.

use bevy::prelude::*;

pub mod animation;
pub mod assets;
pub mod level;

pub fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, animation::plugin, level::plugin));
}
