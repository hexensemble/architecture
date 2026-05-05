use crate::net::config::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::server_sim::*;
use bitcode::{decode, encode};
use renet::{ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use renet_netcode::{
    ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
    ServerConfig,
};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn create_server(
    addr: String,
) -> Result<(RenetServer, NetcodeServerTransport, SocketAddr), Box<dyn std::error::Error>> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let server = RenetServer::new(ConnectionConfig::default());

    let server_socket = UdpSocket::bind(addr)?;
    server_socket.set_nonblocking(true)?;

    let server_addr = server_socket.local_addr()?;

    let server_config = ServerConfig {
        current_time,
        max_clients: MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let server_transport = NetcodeServerTransport::new(server_config, server_socket)?;

    Ok((server, server_transport, server_addr))
}

pub fn update_server(
    server_type: String,
    server: &mut RenetServer,
    server_transport: &mut NetcodeServerTransport,
    sim: &mut ServerSim,
    fixed_dt: f32,
) {
    let dt = Duration::from_secs_f32(fixed_dt);

    server.update(dt);

    if let Err(e) = server_transport.update(dt, server) {
        log::error!("[{server_type}] Server transport update error: {e}");
        return;
    }

    while let Some(event) = server.get_event() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                log::info!("[{server_type}] Client connected: {client_id}");

                sim.spawn_player(client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                log::info!("[{server_type}] Client disconnected: {client_id}, {reason}");

                sim.despawn_player(client_id);
            }
        }
    }

    // Clear old client inputs
    sim.reset_player_velocities();
    // Process new client inputs
    let client_ids: Vec<u64> = server.clients_id_iter().collect();
    for client_id in client_ids {
        while let Some(bytes) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            if let Ok(ClientMessage::Input(input)) = decode(bytes.as_ref()) {
                sim.handle_input(client_id, input);
            }
        }
    }

    let snapshot = sim.step();

    let any_clients = server.clients_id_iter().next().is_some();
    if any_clients {
        let msg = ServerMessage::Snapshot(snapshot);
        server.broadcast_message(DefaultChannel::Unreliable, encode(&msg));
    }

    server_transport.send_packets(server);
}

pub fn create_client(
    addr: String,
    server_addr: SocketAddr,
) -> Result<(RenetClient, NetcodeClientTransport), Box<dyn std::error::Error>> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let client = RenetClient::new(ConnectionConfig::default());

    let client_socket = UdpSocket::bind(addr)?;
    client_socket.set_nonblocking(true)?;

    let authentication = ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id: make_client_id(),
        server_addr,
        user_data: None,
    };

    let client_transport =
        NetcodeClientTransport::new(current_time, authentication, client_socket)?;

    Ok((client, client_transport))
}

pub fn update_client(
    client: &mut RenetClient,
    client_transport: &mut NetcodeClientTransport,
    latest_snapshot: &mut Option<ServerWorldSnapshot>,
    should_disconnect: &mut bool,
    fixed_dt: f32,
) {
    let dt = Duration::from_secs_f32(fixed_dt);

    client.update(dt);

    if let Err(e) = client_transport.update(dt, client) {
        log::error!("[Client] Client transport update error: {}", e);
        *should_disconnect = true;
        return;
    }

    while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
        if let Ok(ServerMessage::Snapshot(snapshot)) = decode(bytes.as_ref()) {
            *latest_snapshot = Some(snapshot);
        }
    }

    if let Err(e) = client_transport.send_packets(client) {
        log::error!("[Client] Send packets error: {}", e);
    }
}
