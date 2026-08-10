use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy_enhanced_input::action::{Action, relationship::Actions};

use super::{components::*, manifest::LoopMode};
use crate::gameplay::character::{
    Character,
    input::{CharacterInput, actions::Move},
    locomotion::components::{AirState, CharacterLandedMessage, Facing, MoveState, Velocity},
};

fn try_play_clip(
    manifest: &super::manifest::CharacterManifestAsset,
    anim_state: &mut CharacterAnimationState,
    clip_name: &str,
) -> bool {
    if let Some(target_idx) = manifest.get_clip_index(clip_name)
        && anim_state.clip_index != target_idx
    {
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
            &mut MoveState,
            &Velocity,
            &Actions<CharacterInput>,
            &mut CharacterAnimationState,
        ),
        With<Character>,
    >,
) {
    let landed_entities: Vec<Entity> = landed_messages.read().map(|msg| msg.0).collect();

    for (entity, manifest_handle, facing, mut move_state, velocity, actions, mut anim_state) in
        &mut query
    {
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

        let is_playing_defend = manifest
            .clips
            .get(anim_state.clip_index as usize)
            .is_some_and(|c| c.name == "defend");

        // Check if movement key is actively held by checking movement action input value
        let has_movement_input = movements
            .iter_many(actions)
            .next()
            .is_some_and(|m| m.abs() > 0.05);

        // Check if CharacterLandedMessage was sent for this entity during touchdown
        let just_landed_message = landed_entities.contains(&entity);

        // TODO: Action Cancel Windows & Combo Chaining
        // - Check if current frame falls within `current_frame.cancel_window` to allow interrupting current animation with attack, jump, or dash inputs.

        // TODO: Camera Shake & Landing Feedback
        // - On `just_landed_message`, compute impact fall velocity and trigger `commands.trigger(CameraShakeEvent { intensity, duration })` for heavy landings.

        // TODO: Audio Triggers
        // - Emit `PlaySfxMessage` / audio commands on landing touchdown or state transitions.

        // 2. State machine transitions
        if move_state.grounded {
            // Defend / Guard animation handling
            if move_state.defending {
                try_play_clip(manifest, &mut anim_state, "defend");
                continue;
            } else if move_state.guard_releasing {
                if is_playing_defend && anim_state.frame_index > 0 {
                    anim_state.frame_index -= 1;
                    anim_state.elapsed_ticks = 0;
                    continue;
                }
                move_state.guard_releasing = false;
            }

            // Dash animation handling
            if move_state.dashing {
                try_play_clip(manifest, &mut anim_state, "dash");
                continue;
            }

            if just_landed_message && try_play_clip(manifest, &mut anim_state, "jump_land") {
                continue;
            }

            if is_playing_jump_land && !anim_state.is_finished() {
                // Keep playing jump_land until landing animation frames 6->7 complete
                continue;
            }

            // Walk vs Idle clip transition
            if has_movement_input && velocity.0.x.abs() > 10.0 {
                try_play_clip(manifest, &mut anim_state, "walk");
            } else {
                try_play_clip(manifest, &mut anim_state, "idle");
            }
        } else {
            // Airborne state (Dash, Jump up, or Fall down)
            if move_state.dashing {
                try_play_clip(manifest, &mut anim_state, "dash");
                continue;
            }

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
                        state.frame_index =
                            u16::try_from(clip.frames.len().saturating_sub(1)).unwrap_or(0);
                        state.set_finished(true);
                    }
                    LoopMode::PingPong => {
                        if clip.frames.len() > 1 {
                            state.frame_index = if state.frame_index == 0 { 1 } else { 0 };
                        } else {
                            state.frame_index = 0;
                        }
                    }
                }
            }

            // TODO: Per-Frame Combat Hitboxes & Hurtboxes
            // - On frame step change, despawn active hitboxes from previous frame and spawn new `Hitbox` colliders for `clip.frames[frame_index]`.
            // - Update `Hurtbox` shapes and attachment offsets.

            // TODO: Frame Audio & Visual Effects (VFX)
            // - Trigger footstep dust particle emitters (`VfxSpawnDef`).
            // - Trigger swing/step sound effects defined on keyframe data (`current_frame.sfx_event`).

            // TODO: Root Motion Support
            // - Extract frame displacement delta and apply directly to character `Velocity` or `Transform`.
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
                // TODO: Sub-pixel position interpolation applied via Bevy transform accumulation
            }
            InterpolationMode::ShaderCrossfade => {
                // TODO: Shader crossfade uniform updates for advanced 2D visual blending between previous and current clips
            }
        }
    }
}
