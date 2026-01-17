use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub const RENDER_WIDTH: u32 = 800;
pub const RENDER_HEIGHT: u32 = 600;

/// Initialize the Bevy app for rendering in the canvas.
/// This function is called by the Leptos `BevyCanvas` component.
pub fn init_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    // "#bevy_canvas" is the default canvas selector used by BevyCanvas
                    canvas: Some("#bevy_canvas".into()),
                    resolution: WindowResolution::new(RENDER_WIDTH, RENDER_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_systems(Startup, setup_scene);

    app
}

/// Sets up a basic 3D scene with a camera and some objects.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn a light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn a cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Spawn a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
}
