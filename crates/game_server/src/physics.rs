//! Server-side game simulation.
//!
//! Uses shared GameSimulationPlugin from game_common.

use bevy::prelude::*;
use game_common::GameSimulationPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(GameSimulationPlugin);
}
