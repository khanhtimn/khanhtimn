use bevy::{asset::AssetMetaCheck, log, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use bevy_enhanced_input::prelude::*;
use game_common::prelude::*;
use iyes_progress::ProgressPlugin;

pub fn run() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::WHITE));

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "assets/game".into(),
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    canvas: Some("#bevy_canvas".into()),
                    desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    );

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{
            EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        };

        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
        ));
    }

    app.add_plugins(
        ProgressPlugin::<GameState>::new()
            .with_state_transition(GameState::Loading, GameState::MainMenu),
    )
    .add_plugins(GamePlugin);

    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron")
            .load_collection::<AudioAssets>()
            .load_collection::<CharacterAssets>(),
    )
    .init_state::<GameState>();

    app.add_systems(Startup, spawn_camera).add_systems(
        OnEnter(GameState::MainMenu),
        (spawn_character, start_background_audio),
    );

    app.run();
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(key = "background_audio")]
    background_audio: Handle<AudioSource>,
}

fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        AudioPlayer(audio_assets.background_audio.clone()),
        PlaybackSettings::LOOP,
    ));
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(key = "naruto_manifest")]
    pub naruto: Handle<CharacterManifestAsset>,
}

fn spawn_character(mut commands: Commands, char_assets: Res<CharacterAssets>) {
    commands.spawn((
        Name::new("Test character"),
        Sprite::default(),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(2.0)),
        CharacterManifestHandle(char_assets.naruto.clone()),
        Character,
        CharacterInput,
        CharacterAnimationState::default(),
        actions!(
            CharacterInput[
                (
                    Action::<actions::Move>::new(),
                    DeadZone::default(),
                    Bindings::spawn((
                        Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                        Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    )),
                ),
                (
                    Action::<actions::Jump>::new(),
                    bindings![KeyCode::Space, KeyCode::KeyK, KeyCode::Numpad2]
                ),
                (
                    Action::<actions::Crouch>::new(),
                    bindings![KeyCode::KeyS, KeyCode::ArrowDown]
                ),
                (
                    Action::<actions::UpModifier>::new(),
                    bindings![KeyCode::KeyW, KeyCode::ArrowUp]
                ),
                (
                    Action::<actions::Dash>::new(),
                    bindings![KeyCode::KeyL, KeyCode::Numpad3]
                ),
            ]
        ),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
