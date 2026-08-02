use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug)]
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

impl Default for MoveStats {
    fn default() -> Self {
        Self {
            ground_speed: 250.0,
            air_speed: 200.0,
            ground_accel: 1500.0,
            air_accel: 800.0,
            friction: 15.0,
            jump_speed: 550.0,
            gravity: 1400.0,
            max_fall_speed: 800.0,
            coyote_time: 0.1,
            jump_buffer_time: 0.1,
        }
    }
}

#[derive(Component, Debug)]
pub struct MoveState {
    pub grounded: bool,
    pub crouching: bool,
    pub just_landed: bool,
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
    pub mode: AirState,
}

impl Default for MoveState {
    fn default() -> Self {
        Self {
            grounded: true,
            crouching: false,
            just_landed: false,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            mode: AirState::Grounded,
        }
    }
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
