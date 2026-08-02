use bevy::{
    ecs::system::{Query, Res},
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

use super::components::{AirState, Facing, MoveState, MoveStats, Velocity};

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

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * dt;
    }
}

pub fn check_ground(mut query: Query<(&mut Transform, &mut Velocity, &mut MoveState)>) {
    for (mut transform, mut velocity, mut state) in &mut query {
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            if velocity.0.y < 0.0 {
                velocity.0.y = 0.0;
            }
            if !state.grounded {
                state.grounded = true;
                state.just_landed = true;
                state.mode = AirState::Grounded;
            }
        }
    }
}

pub fn update_facing(mut query: Query<(&mut Facing, &Velocity)>) {
    for (mut facing, velocity) in &mut query {
        if velocity.0.x > 0.1 {
            *facing = Facing::Right;
        } else if velocity.0.x < -0.1 {
            *facing = Facing::Left;
        }
    }
}
