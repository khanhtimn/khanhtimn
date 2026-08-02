use bevy::{
    ecs::system::{Query, Res},
    time::Time,
};
use bevy_enhanced_input::action::{Action, events::ActionEvents, relationship::Actions};

use crate::gameplay::character::{
    input::{
        CharacterInput,
        actions::{Crouch, Jump, Move},
    },
    locomotion::components::{AirState, MoveState, MoveStats, Velocity},
};

pub fn process_character_input(
    time: Res<Time>,
    jumps: Query<(&Action<Jump>, Option<&ActionEvents>)>,
    movements: Query<&Action<Move>>,
    crouches: Query<&Action<Crouch>>,
    mut players: Query<(
        &mut Velocity,
        &mut MoveState,
        &MoveStats,
        &Actions<CharacterInput>,
    )>,
) {
    let dt = time.delta_secs();

    for (mut velocity, mut state, stats, actions) in &mut players {
        if state.jump_buffer_timer > 0.0 {
            state.jump_buffer_timer -= dt;
        }

        if state.grounded {
            state.coyote_timer = stats.coyote_time;
        } else if state.coyote_timer > 0.0 {
            state.coyote_timer -= dt;
        }

        if let Some((jump, jump_events)) = jumps.iter_many(actions).next() {
            let started = jump_events.is_some_and(|ev| ev.contains(ActionEvents::START));
            if started || **jump {
                state.jump_buffer_timer = stats.jump_buffer_time;
            }
        }

        if let Some(crouch) = crouches.iter_many(actions).next() {
            state.crouching = **crouch;
        }

        if let Some(movement) = movements.iter_many(actions).next() {
            let speed = if state.grounded {
                stats.ground_speed
            } else {
                stats.air_speed
            };
            velocity.0.x = **movement * speed;
        }

        // Execute Jump if buffered and grounded (or within coyote time)
        if state.jump_buffer_timer > 0.0 && (state.grounded || state.coyote_timer > 0.0) {
            velocity.0.y = stats.jump_speed;
            state.grounded = false;
            state.mode = AirState::Rising;
            state.jump_buffer_timer = 0.0;
            state.coyote_timer = 0.0;
        }
    }
}
