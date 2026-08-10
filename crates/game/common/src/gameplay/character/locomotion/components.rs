use bevy::{ecs::message::Message, prelude::*};

/// Message sent when a character touches down on the ground.
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterLandedMessage(pub Entity);

/// Message sent when a character executes a jump impulse (ground jump or double jump).
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterJumpedMessage {
    pub entity: Entity,
    pub is_air_jump: bool,
}

/// Message sent when a character changes facing direction (Left <-> Right).
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterTurnedMessage {
    pub entity: Entity,
    pub facing: Facing,
}

/// Message sent when a character executes a dash impulse.
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterDashedMessage {
    pub entity: Entity,
    pub direction: Vec2,
    pub is_air_dash: bool,
}

/// Message sent when a character enters or exits guard/defend stance.
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterGuardStateChangedMessage {
    pub entity: Entity,
    pub is_guarding: bool,
}

/// Message sent when a defending character blocks an incoming attack.
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterBlockedMessage {
    pub attacker: Entity,
    pub defender: Entity,
    pub damage_blocked: f32,
    pub chip_damage: f32,
}

/// Message sent when a character executes a platform drop through pass-through platforms.
#[derive(Message, Debug, Clone, Copy)]
pub struct CharacterPlatformDroppedMessage(pub Entity);

#[derive(Component, Debug)]
pub struct MoveStats {
    pub ground_speed: f32,
    pub air_speed: f32,
    pub ground_accel: f32,
    pub air_accel: f32,
    pub friction: f32,
    pub jump_speed: f32,
    pub max_jumps: u8,
    pub jump_drift_boost: f32,
    pub air_drift_speed: f32,
    pub dash_speed: f32,
    pub dash_friction: f32,
    pub dash_duration: f32,
    pub dash_cooldown: f32,
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
            max_jumps: 2,
            jump_drift_boost: 150.0,
            air_drift_speed: 320.0,
            dash_speed: 650.0,
            dash_friction: 28.0,
            dash_duration: 0.18,
            dash_cooldown: 0.35,
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
    pub dashing: bool,
    pub defending: bool,
    pub guard_releasing: bool,
    pub jumps_remaining: u8,
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
    pub dash_timer: f32,
    pub dash_cooldown_timer: f32,
    pub mode: AirState,
}

impl Default for MoveState {
    fn default() -> Self {
        Self {
            grounded: true,
            crouching: false,
            dashing: false,
            defending: false,
            guard_releasing: false,
            jumps_remaining: 2,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            dash_timer: 0.0,
            dash_cooldown_timer: 0.0,
            mode: AirState::Grounded,
        }
    }
}

impl MoveState {
    pub fn is_airborne(&self) -> bool {
        !self.grounded || self.mode != AirState::Grounded
    }

    pub fn set_grounded(&mut self, max_jumps: u8) {
        self.grounded = true;
        self.mode = AirState::Grounded;
        self.jumps_remaining = max_jumps;
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
    pub turn_locked: bool,
    pub hitstun_locked: bool,
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
