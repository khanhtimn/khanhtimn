use bevy::ecs::bundle::Bundle;

pub mod components;

use components::*;

#[derive(Bundle, Default)]
pub struct Locomotion {
    pub stats: MoveStats,
    pub state: MoveState,
    pub velocity: Velocity,
    pub locks: Locks,
    pub facing: Facing,
}