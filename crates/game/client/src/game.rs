use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_enhanced_input::prelude::*;
use game_common::prelude::*;

pub fn run() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    canvas: Some("#bevy_canvas".into()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    );

    app.add_plugins(GamePlugin);

    app.add_systems(Startup, (spawn_camera, spawn_character));

    app.run();
}

fn spawn_character(mut commands: Commands) {
    commands.spawn((
        Name::new("Test character"),
        Text2d::new("@"),
        TextFont {
            font_size: FontSize::Px(12.0),
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Character,
        CharacterInput,
        actions!(
            CharacterInput[
                (
                    Action::<actions::Move>::new(),
                    DeadZone::default(),
                    SmoothNudge::default(),
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
