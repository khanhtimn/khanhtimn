//! Player sprite animation.
//!
//! This module handles sprite animation for networked player entities.
//! Animation state is derived from the replicated PlayerState component.

use bevy::prelude::*;
use game_common::components::{Player, PlayerState};
use rand::prelude::*;
use std::time::Duration;

use super::assets::PlayerAssets;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            setup_player_sprites,
            update_animation_timer,
            update_animation_movement,
            update_animation_atlas,
            trigger_step_sound_effect,
        )
            .chain(),
    );
}

/// Component that tracks player's animation state.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    pub frame: usize,
    pub state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq, Clone, Copy, Default)]
pub enum PlayerAnimationState {
    #[default]
    Idling,
    Walking,
}

impl PlayerAnimation {
    const IDLE_FRAMES: usize = 2;
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    const WALKING_FRAMES: usize = 6;
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => 6 + self.frame,
        }
    }
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self::new()
    }
}

/// System that sets up sprite components for newly replicated player entities.
fn setup_player_sprites(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<Entity, (With<Player>, Without<Sprite>)>,
) {
    for entity in &new_players {
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation = PlayerAnimation::new();

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                player_assets.ducky.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation.get_atlas_index(),
                },
            ),
            Transform::from_scale(Vec2::splat(3.0).extend(1.0)),
            animation,
        ));
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state based on replicated state.
fn update_animation_movement(
    mut player_query: Query<(&PlayerState, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (state, mut sprite, mut animation) in &mut player_query {
        let dx = state.velocity.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if state.velocity.x.abs() < 0.1 {
            PlayerAnimationState::Idling
        } else {
            PlayerAnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// Play a step sound effect synchronized with the animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    step_query: Query<&PlayerAnimation>,
) {
    for animation in &step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::rng();
            if let Some(random_step) = player_assets.steps.choose(rng) {
                commands.spawn((
                    AudioPlayer::new(random_step.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}
