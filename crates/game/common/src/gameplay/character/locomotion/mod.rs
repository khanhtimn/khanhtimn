use bevy::prelude::*;

pub mod components;
mod systems;

pub use components::{
    AirState, CharacterBlockedMessage, CharacterDashedMessage, CharacterGuardStateChangedMessage,
    CharacterJumpedMessage, CharacterLandedMessage, CharacterPlatformDroppedMessage,
    CharacterTurnedMessage, Facing, Locks, MoveState, MoveStats, PushVelocity, Velocity,
};

#[derive(Component, Debug, Default)]
#[require(MoveStats, MoveState, Velocity, PushVelocity, Locks, Facing)]
pub struct CharacterLocomotion;

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CharacterLandedMessage>()
            .add_message::<CharacterJumpedMessage>()
            .add_message::<CharacterTurnedMessage>()
            .add_message::<CharacterDashedMessage>()
            .add_message::<CharacterGuardStateChangedMessage>()
            .add_message::<CharacterBlockedMessage>()
            .add_message::<CharacterPlatformDroppedMessage>()
            .add_systems(
                FixedUpdate,
                (
                    systems::apply_gravity,
                    systems::apply_velocity,
                    systems::check_ground,
                    systems::update_facing,
                )
                    .chain(),
            );
    }
}
