use bevy::prelude::*;

use super::components::*;
use crate::gameplay::character::Character;

/// Ticks animation timers and updates character `Sprite` handles.
pub fn animate_character_frames(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Sprite,
            &mut AnimationTimer,
            &mut AnimationFrameIndex,
            &CharacterAnimationMap,
        ),
        With<Character>,
    >,
) {
    for (mut sprite, mut timer, mut index, anim_map) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(frames) = anim_map.animations.get(&anim_map.current_state)
            && !frames.is_empty()
        {
            index.0 = (index.0 + 1) % frames.len();
            sprite.image = frames[index.0].clone();
        }
    }
}
