//! Authoritative game server with WebTransport networking.
//!
//! This binary runs the server-side game logic and accepts
//! WebTransport connections from browser clients.

use bevy::prelude::*;
use bevy_replicon_renet2::RepliconRenetPlugins;
use clap::Parser;
use game_common::{CommonGamePlugin, GameSimulationPlugin, bevy_replicon::prelude::*};

mod connection;

#[derive(Parser, Resource, Debug)]
#[command(name = "game_server")]
pub struct Cli {
    #[arg(short, long, default_value_t = 4433)]
    pub port: u16,

    #[arg(long, default_value = "certs/localhost.pem")]
    pub cert: String,

    #[arg(long, default_value = "certs/localhost-key.pem")]
    pub key: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Starting game server on port {}", cli.port);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(bevy::log::LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "wgpu=error,naga=warn".to_string(),
            ..default()
        })
        .insert_resource(cli)
        // State management
        .add_plugins(bevy::state::app::StatesPlugin)
        // Networking
        .add_plugins((
            RepliconPlugins,
            RepliconRenetPlugins,
            CommonGamePlugin,
            connection::plugin,
        ))
        // Simulation
        .add_plugins(GameSimulationPlugin)
        .run();
}
