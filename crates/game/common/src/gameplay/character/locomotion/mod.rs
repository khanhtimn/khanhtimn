use bevy::app::{App, FixedUpdate, Plugin};
use bevy::ecs::component::Component;

pub mod components;
mod systems;

pub use components::{Facing, Locks, MoveState, MoveStats, PushVelocity, Velocity};

#[derive(Component, Debug, Default)]
#[require(MoveStats, MoveState, Velocity, PushVelocity, Locks, Facing)]
pub struct CharacterLocomotion;

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                systems::apply_gravity,
                systems::apply_velocity,
                systems::check_ground,
                systems::update_facing,
            ),
        );
    }
}
