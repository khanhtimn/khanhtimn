use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug, Default)]
pub struct MoveStats {
    pub ground_speed: f32,
    pub air_speed: f32,
    pub ground_accel: f32,
    pub air_accel: f32,
    pub friction: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub max_fall_speed: f32,
    pub coyote_time: f32,
    pub jump_buffer_time: f32,
}

#[derive(Component, Default, Debug)]
pub struct MoveState {
    pub grounded: bool,
    pub just_landed: bool,
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
    pub mode: AirState,
}

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct PushVelocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Locks {
    pub move_locked: bool,
    pub jump_locked: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AirState {
    #[default]
    Grounded,
    Rising,
    Falling,
}