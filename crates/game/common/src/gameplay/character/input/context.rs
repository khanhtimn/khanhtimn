use bevy::{ecs::system::Query, log, transform::components::Transform};
use bevy_enhanced_input::action::{Action, relationship::Actions};

use crate::gameplay::character::input::actions::{CharacterInput, Jump, Move};

pub fn apply_input(
    jumps: Query<&Action<Jump>>,
    movements: Query<&Action<Move>>,
    mut players: Query<(&mut Transform, &Actions<CharacterInput>)>,
) {
    for (mut transform, actions) in &mut players {
        if let Some(jump) = jumps.iter_many(actions).next()
            && **jump != 0.0
        {
            transform.translation.y += **jump;
        }

        if let Some(movement) = movements.iter_many(actions).next()
            && **movement != 0.0
        {
            transform.translation.x += **movement;
        }
    }
}
