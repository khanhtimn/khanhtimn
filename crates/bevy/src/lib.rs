use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_enhanced_input::prelude::*;

const GROUND_LEVEL: f32 = -200.0;
const GROUND_WIDTH: f32 = 1200.0;
const PLAYER: Vec2 = Vec2::new(50.0, 100.0);
const JUMP_VELOCITY: f32 = 300.0;
const GRAVITY: f32 = 900.0;

pub const RENDER_WIDTH: u32 = 800;
pub const RENDER_HEIGHT: u32 = 600;

pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy_canvas".into()),
                    resolution: WindowResolution::new(RENDER_WIDTH, RENDER_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
        EnhancedInputPlugin,
    ))
    .add_input_context::<Player>()
    .add_systems(Startup, setup)
    .add_systems(Update, calculate_physics)
    .add_observer(apply_movement)
    .add_observer(apply_jump);

    app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Ground
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(GROUND_WIDTH, 5.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.5))),
        Transform::from_translation(Vec3::Y * GROUND_LEVEL),
    ));

    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(PLAYER.x, PLAYER.y))),
        // MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.5))),
        Transform::from_translation(Vec3::Y * (GROUND_LEVEL + 500.0)),
        PlayerPhysics::default(),
        actions!(Player[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(450.0),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp, GamepadButton::South],
            )
        ]),
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
    ));
}

fn apply_movement(movement: On<Fire<Movement>>, mut query: Query<&mut PlayerPhysics>) {
    let mut physics = query.get_mut(movement.context).unwrap();
    physics.velocity.x = movement.value;
}

fn apply_jump(jump: On<Fire<Jump>>, mut query: Query<&mut PlayerPhysics>) {
    let mut physics = query.get_mut(jump.context).unwrap();
    if physics.is_grounded {
        // Jump only if on the ground.
        physics.velocity.y = JUMP_VELOCITY;
        physics.is_grounded = false;
    }
}

fn calculate_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut PlayerPhysics)>) {
    for (mut transform, mut physics) in query.iter_mut() {
        physics.velocity.y -= GRAVITY * time.delta_secs();
        transform.translation.y += physics.velocity.y * time.delta_secs();
        transform.translation.x += physics.velocity.x * time.delta_secs();

        // Prevent moving off screen.
        const MAX_X: f32 = GROUND_WIDTH / 2.0 - PLAYER.x / 2.0;
        transform.translation.x = transform.translation.x.clamp(-MAX_X, MAX_X);

        // Check for ground collision.
        const GROUNDED_Y: f32 = GROUND_LEVEL + PLAYER.y / 2.0;
        if transform.translation.y <= GROUNDED_Y {
            transform.translation.y = GROUNDED_Y;
            physics.velocity.y = 0.0;
            physics.is_grounded = true;
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct PlayerPhysics {
    velocity: Vec2,
    is_grounded: bool,
}

#[derive(Debug, InputAction)]
#[action_output(f32)]
struct Movement;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct Jump;
