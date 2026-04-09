use architecture::core::log::*;
use architecture::net::config::*;
use architecture::net::protocol::message::*;
use architecture::net::server_sim::*;
use architecture::net::stepper::*;
use bitcode::{decode, encode};
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use std::env;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging("dedicated_server.log".to_string())?;

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

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

    // Create server shutdown handler

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        log::info!("[Dedicated Server] Shutting down server...");
        running_clone.store(false, Ordering::SeqCst);
    })?;

    // Start server

    log::info!("[Dedicated Server] Starting server...");

    let mut sim = ServerSim::default();
    sim.reset();

    log::info!("[Dedicated Server] Listening on: {}", server_addr);

    let fixed_dt = sim.fixed_dt();
    let mut server_stepper = ServerStepper::new(fixed_dt, MAX_STEPS_PER_FRAME);

    // Update server

    while running.load(Ordering::SeqCst) {
        server_stepper.wait_and_run(|| {
            let dt = Duration::from_secs_f32(fixed_dt);

            server.update(dt);

            if let Err(e) = server_transport.update(dt, &mut server) {
                log::error!("[Dedicated Server] Server transport update error: {}", e);
                return;
            };

            while let Some(event) = server.get_event() {
                match event {
                    ServerEvent::ClientConnected { client_id } => {
                        log::info!("[Dedicated Server] Client connected: {}", client_id);

                        sim.spawn_player(client_id);
                    }
                    ServerEvent::ClientDisconnected { client_id, reason } => {
                        log::info!(
                            "[Dedicated Server] Client disconnected: {}, {}",
                            client_id,
                            reason
                        );

                        sim.despawn_player(client_id);
                    }
                }
            }

            // Clear old client inputs
            sim.reset_player_velocities();
            // Process new client inputs
            let client_ids: Vec<u64> = server.clients_id_iter().collect();
            for client_id in client_ids {
                while let Some(bytes) =
                    server.receive_message(client_id, DefaultChannel::ReliableOrdered)
                {
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

            server_transport.send_packets(&mut server);
        });
    }

    // Shutdown server

    log::info!("[Dedicated Server] Disconnecting clients...");
    server.disconnect_all();
    // Flush packets
    server_transport.send_packets(&mut server);
    log::info!("[Dedicated Server] Shutdown complete!");

    Ok(())
}
