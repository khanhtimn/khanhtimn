use bevy::{ecs::message::MessageWriter, prelude::*};

use super::components::{
    AirState, CharacterLandedMessage, CharacterTurnedMessage, Facing, Locks, MoveState, MoveStats,
    PushVelocity, Velocity,
};

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut MoveState, &MoveStats)>,
) {
    let dt = time.delta_secs();
    for (mut velocity, mut state, stats) in &mut query {
        if !state.grounded {
            velocity.0.y -= stats.gravity * dt;
            if velocity.0.y < -stats.max_fall_speed {
                velocity.0.y = -stats.max_fall_speed;
            }
            state.mode = if velocity.0.y > 0.0 {
                AirState::Rising
            } else {
                AirState::Falling
            };
        }
    }
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut PushVelocity,
        &MoveState,
        &MoveStats,
        Option<&Locks>,
    )>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut velocity, mut push_vel, state, stats, locks) in &mut query {
        let is_hitstun = locks.is_some_and(|l| l.hitstun_locked);

        // Combine locomotion velocity and knockback push velocity
        let total_vel = velocity.0 + push_vel.0;
        transform.translation += Vec3::new(total_vel.x, total_vel.y, 0.0) * dt;

        // Apply fast friction decay to high-velocity dash impulses
        if state.dashing && !is_hitstun {
            let target_speed = stats.ground_speed;
            let current_abs = velocity.0.x.abs();
            if current_abs > target_speed {
                let sign = velocity.0.x.signum();
                let new_abs = (current_abs - stats.dash_friction * 50.0 * dt).max(target_speed);
                velocity.0.x = sign * new_abs;
            }
        }

        // Apply knockback decay to PushVelocity
        if push_vel.0.length_squared() > 0.001 {
            let decay = stats.friction * 50.0 * dt;
            let current_len = push_vel.0.length();
            if current_len <= decay {
                push_vel.0 = Vec2::ZERO;
            } else {
                push_vel.0 *= (current_len - decay) / current_len;
            }
        }
    }
}

pub fn check_ground(
    mut landed_writer: MessageWriter<CharacterLandedMessage>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &mut MoveState,
        &MoveStats,
    )>,
) {
    for (entity, mut transform, mut velocity, mut state, stats) in &mut query {
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            if velocity.0.y < 0.0 {
                velocity.0.y = 0.0;
            }
            if !state.grounded {
                state.grounded = true;
                state.mode = AirState::Grounded;
                state.jumps_remaining = stats.max_jumps;
                landed_writer.write(CharacterLandedMessage(entity));
            }
        }
    }
}

pub fn update_facing(
    mut turned_writer: MessageWriter<CharacterTurnedMessage>,
    mut query: Query<(Entity, &mut Facing, &Velocity, Option<&Locks>)>,
) {
    for (entity, mut facing, velocity, locks) in &mut query {
        if locks.is_some_and(|l| l.turn_locked || l.hitstun_locked) {
            continue;
        }

        if velocity.0.x > 0.1 && *facing != Facing::Right {
            *facing = Facing::Right;
            turned_writer.write(CharacterTurnedMessage {
                entity,
                facing: Facing::Right,
            });
        } else if velocity.0.x < -0.1 && *facing != Facing::Left {
            *facing = Facing::Left;
            turned_writer.write(CharacterTurnedMessage {
                entity,
                facing: Facing::Left,
            });
        }
    }
}
