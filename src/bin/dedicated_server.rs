use architecture::net::config::*;
use architecture::net::protocol::message::ServerMessage;
use architecture::net::server_sim::ServerSim;
use bitcode::encode;
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use std::env;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

    //TODO: Needs to be command line argument
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:27960".to_string());

    // Create server

    let mut server = RenetServer::new(ConnectionConfig::default());

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

    let mut server_transport = NetcodeServerTransport::new(server_config, server_socket)?;

    // Start server

    let mut sim = ServerSim::default();
    sim.reset();

    let fixed_dt = sim.fixed_dt();

    println!("[Dedicated Server] Listening on: {}.", server_addr);

    loop {
        let dt = Duration::from_secs_f32(fixed_dt);

        server.update(dt);

        server_transport.update(dt, &mut server)?;

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("[Dedicated Server] Client connected: {}", client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!(
                        "[Dedicated Server] Client disconnected: {}, {}",
                        client_id, reason
                    );
                }
            }
        }

        let snapshot = sim.step();

        let any_clients = server.clients_id_iter().next().is_some();
        if any_clients {
            let msg = ServerMessage::Snapshot(snapshot);
            server.broadcast_message(DefaultChannel::Unreliable, encode(&msg));
        }

        server_transport.send_packets(&mut server);

        std::thread::sleep(dt);
    }
}
