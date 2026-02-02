//! WebTransport client connection setup for multiplayer mode.
//!
//! This module handles establishing and managing the connection
//! to the game server using WebTransport protocol.
//! Only active when GameMode is Multiplayer.

use bevy::prelude::*;
use bevy_renet2::netcode::{WebServerDestination, WebTransportClient, WebTransportClientConfig};
use bevy_replicon_renet2::{
    RenetChannelsExt,
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet2::{ConnectionConfig, RenetClient},
};
use game_common::{bevy_replicon::prelude::*, protocol::PROTOCOL_ID};

use crate::{
    ServerConfig,
    screens::{GameMode, GameScreen},
};

pub fn plugin(app: &mut App) {
    app.init_state::<ConnectionState>()
        .add_systems(OnEnter(GameScreen::Connecting), setup_connection)
        .add_systems(
            Update,
            (monitor_connection, log_connection_state)
                .run_if(resource_equals(GameMode::Multiplayer)),
        );
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

fn log_connection_state(state: Res<State<ConnectionState>>) {
    if state.is_changed() {
        info!("[Connection] State: {:?}", state.get());
    }
}

fn setup_connection(
    mut commands: Commands,
    channels: Res<RepliconChannels>,
    server_config: Res<ServerConfig>,
    mut next_state: ResMut<NextState<ConnectionState>>,
) {
    info!("[Connection] Setting up WebTransport connection...");

    let client_id = uuid::Uuid::new_v4().as_u64_pair().0;
    info!("[Connection] Client ID: {}", client_id);

    let client = RenetClient::new(
        ConnectionConfig::from_channels(channels.server_configs(), channels.client_configs()),
        false,
    );

    let server_url = &server_config.url;
    info!("[Connection] Server URL: {}", server_url.as_str());

    let server_dest = WebServerDestination::from(server_url.clone());

    let server_addr: std::net::SocketAddr = server_dest.clone().into();
    info!("[Connection] Server address (hashed): {:?}", server_addr);
    info!("[Connection] Using PKI certificate validation");
    let wt_config = WebTransportClientConfig::new(server_dest);
    let wt_client = WebTransportClient::new(wt_config);

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        socket_id: 0,
        server_addr,
        user_data: None,
    };

    let current_time = std::time::Duration::from_millis(js_sys::Date::now() as u64);
    info!(
        "[Connection] Current time (Unix ms): {}",
        current_time.as_millis()
    );

    match NetcodeClientTransport::new(current_time, authentication, wt_client) {
        Ok(transport) => {
            commands.insert_resource(client);
            commands.insert_resource(transport);
            next_state.set(ConnectionState::Connecting);
            info!("[Connection] Connecting to server: {}", server_url);
        }
        Err(e) => {
            error!("[Connection] Failed to create transport: {:?}", e);
            next_state.set(ConnectionState::Error);
        }
    }
}

fn monitor_connection(
    client: Option<Res<RenetClient>>,
    mut next_conn_state: ResMut<NextState<ConnectionState>>,
    mut next_game_state: ResMut<NextState<GameScreen>>,
    conn_state: Res<State<ConnectionState>>,
) {
    let Some(client) = client else { return };

    match conn_state.get() {
        ConnectionState::Connecting => {
            if client.is_connected() {
                info!("[Connection] Connected to server!");
                next_conn_state.set(ConnectionState::Connected);
                next_game_state.set(GameScreen::Playing);
            } else if client.is_disconnected() {
                warn!("[Connection] Failed to connect to server");
                next_conn_state.set(ConnectionState::Disconnected);
                next_game_state.set(GameScreen::MainMenu);
            }
        }
        ConnectionState::Connected => {
            if client.is_disconnected() {
                warn!("[Connection] Disconnected from server");
                next_conn_state.set(ConnectionState::Disconnected);
                next_game_state.set(GameScreen::Disconnected);
            }
        }
        _ => {}
    }
}
