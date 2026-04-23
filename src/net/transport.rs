use crate::net::config::*;
use renet::{ConnectionConfig, RenetClient, RenetServer};
use renet_netcode::{
    ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
    ServerConfig,
};
use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};

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
