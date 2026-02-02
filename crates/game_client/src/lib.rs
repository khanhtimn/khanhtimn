//! Bevy game client for WASM with WebTransport networking.
//!
//! This crate provides the client-side game logic that runs in the browser.
//! It connects to the game server via WebTransport and renders the game state.

// This crate only compiles to WASM - WebTransport client requires browser APIs
#[cfg(not(target_family = "wasm"))]
compile_error!("game_client only supports WASM targets. Use `--target wasm32-unknown-unknown`.");

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_replicon_renet2::RepliconRenetPlugins;
use game_common::{CommonGamePlugin, bevy_replicon::prelude::*};

mod audio;
mod connection;
mod demo;
mod input;
mod menu;
mod screens;
mod singleplayer;
mod theme;

pub use screens::{GameMode, GameScreen};

/// Configuration for connecting to the game server.
#[derive(Resource, Clone)]
pub struct ServerConfig {
    /// The WebTransport URL to connect to.
    pub url: url::Url,
}

/// High-level groupings of systems for the app in the `Update` schedule.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else.
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

/// Initialize the Bevy app with a server URL.
///
/// The URL should be provided from environment configuration via Leptos.
pub fn init_bevy_app(server_url: String) -> App {
    let config = ServerConfig {
        url: url::Url::parse(&server_url).expect("Invalid server URL"),
    };
    init_bevy_app_with_config(config)
}

/// Initialize the Bevy app with full server configuration.
pub fn init_bevy_app_with_config(config: ServerConfig) -> App {
    let mut app = App::new();

    // Store server config
    app.insert_resource(config);

    // Bevy plugins with WASM-friendly configuration
    app.add_plugins(
        DefaultPlugins
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

    // Networking plugins
    app.add_plugins((RepliconPlugins, RepliconRenetPlugins, CommonGamePlugin));

    // Game plugins - order matters!
    // singleplayer adds EnhancedInputPlugin, so it must come before input
    app.add_plugins((
        audio::plugin,
        screens::plugin,
        menu::plugin,
        singleplayer::plugin,
        input::plugin,
        connection::plugin,
        theme::plugin,
        demo::plugin,
    ));

    // Configure system sets
    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AppSystems::RecordInput,
            AppSystems::Update,
        )
            .chain(),
    );

    // Pause state
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    // Camera
    app.add_systems(Startup, spawn_camera);

    app
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
