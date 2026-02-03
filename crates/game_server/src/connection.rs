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
use game_common::{
    bevy_replicon::prelude::*,
    protocol::{MAX_CLIENTS, PROTOCOL_ID},
};
use rustls_pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use std::{net::SocketAddr, time::SystemTime};

use crate::Cli;

#[derive(Resource)]
pub struct TokioRuntime(pub tokio::runtime::Runtime);

pub fn plugin(app: &mut App) {
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

    let bind_addr = SocketAddr::new(cli.host, cli.port);

    // Use public URL if provided, otherwise construct from bind address
    let public_url = cli
        .public_url
        .clone()
        .unwrap_or_else(|| format!("https://{}:{}/", cli.host, cli.port));

    let client_url: url::Url = public_url.parse().expect("Invalid public URL");
    let server_dest = bevy_replicon_renet2::netcode::WebServerDestination::from(client_url.clone());

    let hashed_addr: SocketAddr = server_dest.into();
    info!("Public URL: {}", client_url);
    info!("Server hashed address (for netcode): {:?}", hashed_addr);

    let server_config = ServerSetupConfig {
        current_time,
        max_clients: MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        socket_addresses: vec![vec![hashed_addr]],
    };

    // Load certificate and key from files
    let cert = load_cert(&cli.cert);
    let key = load_key(&cli.key);

    let wt_config = WebTransportServerConfig {
        cert,
        key,
        listen: bind_addr,
        max_clients: MAX_CLIENTS,
    };

    info!("Loaded TLS certificate from: {}", cli.cert);
    info!("Loaded TLS private key from: {}", cli.key);

    let wt_server = WebTransportServer::new(wt_config, runtime.0.handle().clone())
        .expect("Failed to create WebTransport server");

    let transport = NetcodeServerTransport::new(server_config, wt_server)
        .expect("Failed to create server transport");

    commands.insert_resource(server);
    commands.insert_resource(transport);

    info!("WebTransport server listening on {}", bind_addr);
}

fn load_cert(path: &str) -> CertificateDer<'static> {
    let certs: Vec<_> = CertificateDer::pem_file_iter(path)
        .expect("Failed to read certificate file")
        .collect::<Result<_, _>>()
        .expect("Failed to parse certificates");

    if certs.is_empty() {
        panic!("No certificates found in '{}'", path);
    }

    certs.into_iter().next().unwrap().into_owned()
}

fn load_key(path: &str) -> PrivateKeyDer<'static> {
    let key = PrivateKeyDer::from_pem_file(path).unwrap_or_else(|e| {
        panic!("Failed to parse private key PEM '{}': {}", path, e);
    });

    key.clone_key()
}
