//! Authoritative game server with WebTransport networking.
//!
//! This binary runs the server-side game logic and accepts
//! WebTransport connections from browser clients.

use bevy::prelude::*;
use bevy_replicon_renet2::RepliconRenetPlugins;
use clap::Parser;
use khanhtimn_dev_common::{bevy_replicon::prelude::*, CommonGamePlugin};

mod connection;
mod physics;
mod spawning;

/// Command-line arguments for the game server.
#[derive(Parser, Resource, Debug)]
#[command(name = "game_server")]
#[command(about = "Authoritative game server with WebTransport")]
pub struct Cli {
    /// Port to listen on for WebTransport connections.
    #[arg(short, long, default_value_t = 4433)]
    pub port: u16,

    /// Path to TLS certificate file (PEM format).
    /// Required for WebTransport (HTTPS).
    #[arg(long, default_value = "certs/cert.pem")]
    pub cert: String,

    /// Path to TLS private key file (PEM format).
    /// Required for WebTransport (HTTPS).
    #[arg(long, default_value = "certs/key.pem")]
    pub key: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Starting game server on port {}", cli.port);

    App::new()
        // Minimal plugins for headless server
        .add_plugins(MinimalPlugins)
        // Add logging for debug output
        .add_plugins(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "wgpu=error,naga=warn".to_string(),
            ..default()
        })
        // State management (required by bevy_replicon)
        .add_plugins(bevy::state::app::StatesPlugin)
        // Networking
        .add_plugins((RepliconPlugins, RepliconRenetPlugins, CommonGamePlugin))
        // Server configuration
        .insert_resource(cli)
        // Server-specific plugins
        .add_plugins((connection::plugin, physics::plugin, spawning::plugin))
        .run();
}
