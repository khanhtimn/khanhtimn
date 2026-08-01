use bevy::{asset::AssetMetaCheck, log, prelude::*};

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

    app.add_plugins(game_common::GamePlugin);

    app.add_systems(Startup, (spawn_camera, spawn_character));

    app.run();
}

fn spawn_character(mut commands: Commands) {
    use game_common::gameplay::character::*;
    use bevy_enhanced_input::prelude::*;
    // log::info!("Spawning character...");
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
        Character::default(),
        actions!(
            input::actions::CharacterInput[
                (
                    Action::<input::actions::Move>::new(),
                    // Conditions and modifiers as components.
                    DeadZone::default(), // Apply non-uniform normalization that works for both digital and analog inputs, otherwise diagonal movement will be faster.
                    SmoothNudge::default(), // Apply smoothing.
                    DeltaScale::default(), // Multiply by delta time to make it framerate-independent.
                    Scale::splat(200.0), // Additionally multiply by a constant to achieve the desired speed.
                    // Bindings are entities related to actions.
                    // An action can have multiple bindings and will respond to any of them.
                    Bindings::spawn((
                        Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                        Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    )),
                ),
                (
                    Action::<input::actions::Jump>::new(),
                    SmoothNudge::default(),
                    DeltaScale::default(),
                    Scale::splat(200.0),
                    bindings![KeyCode::KeyK, KeyCode::Numpad2, KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp]
                ),
                (
                    Action::<input::actions::Dash>::new(),
                    DeltaScale::default(),
                    bindings![KeyCode::KeyL, KeyCode::Numpad3]
                ),
        ])
        
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
