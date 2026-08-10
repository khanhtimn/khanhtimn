use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy_enhanced_input::action::{relationship::Actions, Action};

use super::{components::*, manifest::LoopMode};
use crate::gameplay::character::{
    input::{actions::Move, CharacterInput},
    locomotion::components::{AirState, CharacterLandedMessage, Facing, MoveState, Velocity},
    Character,
};

fn try_play_clip(
    manifest: &super::manifest::CharacterManifestAsset,
    anim_state: &mut CharacterAnimationState,
    clip_name: &str,
) -> bool {
    if let Some(target_idx) = manifest.get_clip_index(clip_name)
        && anim_state.clip_index != target_idx {
            anim_state.clip_index = target_idx;
            anim_state.frame_index = 0;
            anim_state.elapsed_ticks = 0;
            anim_state.flags.remove(AnimationPlaybackFlags::IS_FINISHED);
            return true;
        }
    false
}

/// Synchronizes character locomotion (Velocity, Grounded state, Facing, Landed Message) to presentation animation state.
pub fn update_character_animation_state(
    manifest_assets: Res<Assets<super::manifest::CharacterManifestAsset>>,
    movements: Query<&Action<Move>>,
    mut landed_messages: MessageReader<CharacterLandedMessage>,
    mut query: Query<
        (
            Entity,
            &CharacterManifestHandle,
            &Facing,
            &MoveState,
            &Velocity,
            &Actions<CharacterInput>,
            &mut CharacterAnimationState,
        ),
        With<Character>,
    >,
) {
    let landed_entities: Vec<Entity> = landed_messages.read().map(|msg| msg.0).collect();

    for (entity, manifest_handle, facing, move_state, velocity, actions, mut anim_state) in &mut query {
        let Some(manifest) = manifest_assets.get(&manifest_handle.0) else {
            continue;
        };

        // 1. Sync Facing direction to FLIP_X flag
        match facing {
            Facing::Left => anim_state.flags.insert(AnimationPlaybackFlags::FLIP_X),
            Facing::Right => anim_state.flags.remove(AnimationPlaybackFlags::FLIP_X),
        }

        // Get current playing clip name for state machine checking
        let is_playing_jump_land = manifest
            .clips
            .get(anim_state.clip_index as usize)
            .is_some_and(|c| c.name == "jump_land");

        // Check if movement key is actively held by checking movement action input value
        let has_movement_input = movements
            .iter_many(actions)
            .next()
            .is_some_and(|m| m.abs() > 0.05);

        // Check if CharacterLandedMessage was sent for this entity during touchdown
        let just_landed_message = landed_entities.contains(&entity);

        // 2. State machine transitions
        if move_state.grounded {
            if just_landed_message
                && try_play_clip(manifest, &mut anim_state, "jump_land") {
                    continue;
                }

            if is_playing_jump_land && !anim_state.is_finished() {
                // Keep playing jump_land until landing animation frames 6->7 complete
                continue;
            }

            // Option A: Only play walk clip when movement key is actively held AND velocity > 10.0
            if has_movement_input && velocity.0.x.abs() > 10.0 {
                try_play_clip(manifest, &mut anim_state, "walk");
            } else {
                try_play_clip(manifest, &mut anim_state, "idle");
            }
        } else {
            // Airborne state (Jumping up or Falling down)
            if velocity.0.y > 0.0 || move_state.mode == AirState::Rising {
                try_play_clip(manifest, &mut anim_state, "jump_up");
            } else {
                try_play_clip(manifest, &mut anim_state, "jump_down");
            }
        }
    }
}

/// Advances character animation frames in FixedUpdate (60Hz timestep).
pub fn advance_character_animations(
    manifest_assets: Res<Assets<super::manifest::CharacterManifestAsset>>,
    mut query: Query<(&CharacterManifestHandle, &mut CharacterAnimationState), With<Character>>,
) {
    for (manifest_handle, mut state) in &mut query {
        // 1. Handle hitstop / freeze-frame impact pauses
        if state.hitstop_ticks > 0 {
            state.hitstop_ticks -= 1;
            continue;
        }

        // 2. Check paused state
        if state.is_paused() {
            continue;
        }

        let Some(manifest) = manifest_assets.get(&manifest_handle.0) else {
            continue;
        };
        let Some(clip) = manifest.clips.get(state.clip_index as usize) else {
            continue;
        };

        if clip.frames.is_empty() {
            continue;
        }

        // 3. Check completion state for non-looping clips
        if state.is_finished()
            && (clip.loop_mode == LoopMode::Once || clip.loop_mode == LoopMode::HoldLast)
        {
            continue;
        }

        // 4. Increment tick counter for current frame
        state.elapsed_ticks += 1;

        let current_frame = &clip.frames[state.frame_index as usize];

        // 5. Advance frame if elapsed_ticks reaches target duration_ticks
        if state.elapsed_ticks >= current_frame.duration_ticks {
            state.elapsed_ticks = 0;

            if (state.frame_index as usize) + 1 < clip.frames.len() {
                state.frame_index += 1;
            } else {
                match clip.loop_mode {
                    LoopMode::Repeat => {
                        state.frame_index = 0;
                    }
                    LoopMode::Once => {
                        state.set_finished(true);
                    }
                    LoopMode::HoldLast => {
                        state.frame_index = (clip.frames.len() - 1) as u16;
                        state.set_finished(true);
                    }
                    LoopMode::PingPong => {
                        // Glue Code: Reserved for ping-pong animation support in future pass
                    }
                }
            }
        }
    }
}

/// Updates character sprite atlas indices, texture sheets, and applies visual settings in PostUpdate.
pub fn update_character_sprites(
    manifest_assets: Res<Assets<super::manifest::CharacterManifestAsset>>,
    video_settings: Res<AnimationVideoSettings>,
    mut query: Query<
        (
            &CharacterManifestHandle,
            &CharacterAnimationState,
            &mut Sprite,
        ),
        With<Character>,
    >,
) {
    for (manifest_handle, anim_state, mut sprite) in &mut query {
        let Some(manifest) = manifest_assets.get(&manifest_handle.0) else {
            continue;
        };
        let Some(clip) = manifest.clips.get(anim_state.clip_index as usize) else {
            continue;
        };

        let loaded_sheet = manifest
            .loaded_sheets
            .get(&clip.sheet)
            .or_else(|| manifest.loaded_sheets.values().next());

        let Some(sheet) = loaded_sheet else {
            continue;
        };

        if let Some(frame) = clip.frames.get(anim_state.frame_index as usize) {
            sprite.image = sheet.image_handle.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: sheet.atlas_layout_handle.clone(),
                index: frame.sprite_index,
            });

            sprite.flip_x = anim_state.flags.contains(AnimationPlaybackFlags::FLIP_X);
            sprite.flip_y = anim_state.flags.contains(AnimationPlaybackFlags::FLIP_Y);
        }

        match video_settings.interpolation_mode {
            InterpolationMode::Discrete60Hz => {
                // Strict 60Hz rendering - no extra visual smoothing applied
            }
            InterpolationMode::TransformInterpolated => {
                // Sub-pixel position interpolation applied via Bevy transform accumulation
            }
            InterpolationMode::ShaderCrossfade => {
                // Glue Code: Shader crossfade uniform updates for advanced 2D visual blending
            }
        }
    }
}
