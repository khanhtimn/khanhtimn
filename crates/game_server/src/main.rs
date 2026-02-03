//! Authoritative game server with WebTransport networking.
//!
//! This binary runs the server-side game logic and accepts
//! WebTransport connections from browser clients.

use bevy::prelude::*;
use bevy_replicon_renet2::RepliconRenetPlugins;
use clap::Parser;
use game_common::{CommonGamePlugin, GameSimulationPlugin, bevy_replicon::prelude::*};
use std::net::IpAddr;

mod connection;

#[derive(Parser, Resource, Debug)]
#[command(name = "game_server")]
pub struct Cli {
    /// Host address to bind to
    #[arg(long, default_value = "0.0.0.0", env = "GAME_SERVER_HOST")]
    pub host: IpAddr,

    /// Port to listen on
    #[arg(short, long, default_value_t = 4433, env = "GAME_SERVER_PORT")]
    pub port: u16,

    /// Public URL for clients to connect (used for netcode address hashing)
    /// Shared with Leptos SSR to ensure client and server use the same URL
    #[arg(long, env = "GAME_SERVER_URL")]
    pub public_url: Option<String>,

    /// Path to TLS certificate (PEM format)
    #[arg(long, default_value = "certs/localhost.pem", env = "GAME_SERVER_CERT")]
    pub cert: String,

    /// Path to TLS private key (PEM format)
    #[arg(
        long,
        default_value = "certs/localhost-key.pem",
        env = "GAME_SERVER_KEY"
    )]
    pub key: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Starting game server on {}:{}", cli.host, cli.port);
    if let Some(ref url) = cli.public_url {
        println!("Public URL: {}", url);
    }

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
