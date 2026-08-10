use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;
use bevy_enhanced_input::action::{Action, relationship::Actions};

use crate::gameplay::character::{
    input::{
        CharacterInput,
        actions::{Crouch, Dash, Jump, Move, PlatformDrop},
    },
    locomotion::components::{
        AirState, CharacterDashedMessage, CharacterGuardStateChangedMessage,
        CharacterJumpedMessage, CharacterPlatformDroppedMessage, Facing, Locks, MoveState,
        MoveStats, Velocity,
    },
};

#[allow(clippy::too_many_arguments)]
pub fn process_character_input(
    time: Res<Time>,
    mut jumped_writer: MessageWriter<CharacterJumpedMessage>,
    mut dashed_writer: MessageWriter<CharacterDashedMessage>,
    mut guard_writer: MessageWriter<CharacterGuardStateChangedMessage>,
    mut drop_writer: MessageWriter<CharacterPlatformDroppedMessage>,
    jumps: Query<&Action<Jump>>,
    dashes: Query<&Action<Dash>>,
    crouches: Query<&Action<Crouch>>,
    platform_drops: Query<&Action<PlatformDrop>>,
    movements: Query<&Action<Move>>,
    mut players: Query<(
        Entity,
        &mut Velocity,
        &mut MoveState,
        &MoveStats,
        &Facing,
        Option<&Locks>,
        &Actions<CharacterInput>,
    )>,
) {
    let dt = time.delta_secs();

    for (entity, mut velocity, mut state, stats, facing, locks, actions) in &mut players {
        let is_hitstun = locks.is_some_and(|l| l.hitstun_locked);
        let is_move_locked = locks.is_some_and(|l| l.move_locked) || is_hitstun;
        let is_jump_locked = locks.is_some_and(|l| l.jump_locked) || is_hitstun;

        // Update timers
        if state.jump_buffer_timer > 0.0 {
            state.jump_buffer_timer -= dt;
        }

        if state.dash_timer > 0.0 {
            state.dash_timer -= dt;
            if state.dash_timer <= 0.0 {
                state.dashing = false;
            }
        }

        if state.dash_cooldown_timer > 0.0 {
            state.dash_cooldown_timer -= dt;
        }

        if state.grounded {
            state.coyote_timer = stats.coyote_time;
        } else if state.coyote_timer > 0.0 {
            state.coyote_timer -= dt;
        }

        // Process Platform Drop input (Chord of Crouch + Jump)
        if let Some(drop) = platform_drops.iter_many(actions).next()
            && **drop
            && !is_move_locked
        {
            log::info!("Dropped from platform");
            drop_writer.write(CharacterPlatformDroppedMessage(entity));
        }

        // Process Crouch input (S Key - behaves as Defend/Guard stance for now)
        let was_defending = state.defending;
        if let Some(crouch) = crouches.iter_many(actions).next() {
            let is_crouch_held = **crouch;
            state.crouching = is_crouch_held;

            if is_crouch_held && !is_move_locked && state.grounded {
                state.defending = true;
                state.guard_releasing = false;
                velocity.0.x = 0.0;
            } else if state.defending {
                state.defending = false;
                state.guard_releasing = true;
            }
        }

        if state.defending != was_defending {
            guard_writer.write(CharacterGuardStateChangedMessage {
                entity,
                is_guarding: state.defending,
            });
        }

        // Process Dash input (L Key)
        if let Some(dash) = dashes.iter_many(actions).next()
            && **dash
            && !is_move_locked
            && !state.defending
            && state.dash_cooldown_timer <= 0.0
        {
            let facing_sign = match facing {
                Facing::Right => 1.0,
                Facing::Left => -1.0,
            };
            let input_dir = movements
                .iter_many(actions)
                .next()
                .map_or(facing_sign, |m| {
                    if m.abs() > 0.05 {
                        m.signum()
                    } else {
                        facing_sign
                    }
                });

            velocity.0.x = input_dir * stats.dash_speed;
            state.dashing = true;
            state.dash_timer = stats.dash_duration;
            state.dash_cooldown_timer = stats.dash_cooldown;

            dashed_writer.write(CharacterDashedMessage {
                entity,
                direction: Vec2::new(input_dir, 0.0),
                is_air_dash: !state.grounded,
            });
        }

        // Process Move input
        let mut raw_move_input = 0.0;
        if let Some(movement) = movements.iter_many(actions).next() {
            raw_move_input = **movement;
            if !is_move_locked && !state.defending && !state.dashing {
                let speed = if state.grounded {
                    stats.ground_speed
                } else {
                    stats.air_speed
                };
                velocity.0.x = raw_move_input * speed;
            }
        }

        // Process Jump input buffer
        if let Some(jump) = jumps.iter_many(actions).next()
            && **jump
            && !is_jump_locked
            && !state.defending
        {
            state.jump_buffer_timer = stats.jump_buffer_time;
        }

        // Execute Jump (Ground Jump or Double Jump)
        let can_ground_jump = state.grounded || state.coyote_timer > 0.0;
        let can_air_jump = !can_ground_jump && state.jumps_remaining > 0;

        if state.jump_buffer_timer > 0.0 && !is_jump_locked && (can_ground_jump || can_air_jump) {
            velocity.0.y = stats.jump_speed;
            state.jump_buffer_timer = 0.0;
            state.coyote_timer = 0.0;

            let input_sign = if raw_move_input.abs() > 0.05 {
                raw_move_input.signum()
            } else {
                0.0
            };

            if can_ground_jump {
                // Ground Jump Physics: Preserve running momentum + inject instant drift boost
                if input_sign != 0.0 {
                    velocity.0.x += input_sign * stats.jump_drift_boost;
                }
                state.grounded = false;
                state.mode = AirState::Rising;
                state.jumps_remaining = stats.max_jumps.saturating_sub(1);

                jumped_writer.write(CharacterJumpedMessage {
                    entity,
                    is_air_jump: false,
                });
            } else {
                // Air Jump / Double Jump Physics: Instant horizontal redirection
                if input_sign != 0.0 {
                    velocity.0.x = input_sign * stats.air_drift_speed;
                }
                state.mode = AirState::Rising;
                state.jumps_remaining = state.jumps_remaining.saturating_sub(1);

                jumped_writer.write(CharacterJumpedMessage {
                    entity,
                    is_air_jump: true,
                });
            }
        }
    }
}
