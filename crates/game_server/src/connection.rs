//! WebTransport server connection setup.
//!
//! This module handles setting up the WebTransport server
//! that accepts connections from browser clients.

use bevy::prelude::*;
use bevy_replicon_renet2::{
    RenetChannelsExt,
    netcode::{
        NetcodeServerTransport, ServerAuthentication, ServerSetupConfig, WebTransportServer,
        WebTransportServerConfig,
    },
    renet2::{ConnectionConfig, RenetServer},
};
use khanhtimn_dev_common::{
    bevy_replicon::prelude::*,
    protocol::{MAX_CLIENTS, PROTOCOL_ID},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::SystemTime,
};

use crate::Cli;

/// Tokio runtime resource for async operations.
#[derive(Resource)]
pub struct TokioRuntime(pub tokio::runtime::Runtime);

pub fn plugin(app: &mut App) {
    // Create tokio runtime for WebTransport
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    app.insert_resource(TokioRuntime(runtime));
    app.add_systems(Startup, setup_server);
}

fn setup_server(
    mut commands: Commands,
    channels: Res<RepliconChannels>,
    cli: Res<Cli>,
    runtime: Res<TokioRuntime>,
) {
    let server = RenetServer::new(ConnectionConfig::from_channels(
        channels.server_configs(),
        channels.client_configs(),
    ));

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time before UNIX epoch");

    // The bind address for the WebTransport server
    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), cli.port);

    // The client's connection URL - this gets hashed to create the "server address" for netcode
    // Using 127.0.0.1 explicitly to avoid IPv6 resolution issues with "localhost"
    let client_url: url::Url = format!("https://127.0.0.1:{}/", cli.port)
        .parse()
        .expect("Invalid URL");
    let server_dest = bevy_replicon_renet2::netcode::WebServerDestination::from(client_url.clone());

    // Convert the WebServerDestination to a SocketAddr - this is the HASHED address
    // that the client will use. The server MUST use this same address in socket_addresses!
    let hashed_addr: SocketAddr = server_dest.clone().into();
    println!("Server hashed address (for netcode): {:?}", hashed_addr);

    let server_config = ServerSetupConfig {
        current_time,
        max_clients: MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        // Use the HASHED address here - must match what client sends!
        socket_addresses: vec![vec![hashed_addr]],
    };

    // Create a self-signed WebTransport server config
    // This generates a temporary certificate valid for ~2 weeks
    let proxies = vec![bind_addr.into(), server_dest];

    let (wt_config, cert_hash) =
        WebTransportServerConfig::new_selfsigned_with_proxies(bind_addr, proxies, MAX_CLIENTS)
            .expect("Failed to create self-signed WebTransport config");

    println!("Server certificate hash: {:?}", cert_hash);

    let wt_server = WebTransportServer::new(wt_config, runtime.0.handle().clone())
        .expect("Failed to create WebTransport server");

    let transport = NetcodeServerTransport::new(server_config, wt_server)
        .expect("Failed to create server transport");

    commands.insert_resource(server);
    commands.insert_resource(transport);

    println!(
        "WebTransport server listening on https://127.0.0.1:{}",
        cli.port
    );
}
