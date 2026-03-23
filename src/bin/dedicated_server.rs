use architecture::net::config::*;
use architecture::net::protocol::message::*;
use architecture::net::server_sim::*;
use architecture::net::stepper::*;
use bitcode::encode;
use log::LevelFilter;
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, WriteLogger,
};
use std::env;
use std::fs::{self, OpenOptions};
use std::net::UdpSocket;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn innit_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = PathBuf::from("logs");
    fs::create_dir_all(&log_dir)?;
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("dedicated_server.log"))?;

    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Info)
        .set_level_padding(LevelPadding::Right)
        .build();

    let file_logger = WriteLogger::new(LevelFilter::Info, config.clone(), log_file);

    let term_logger = TermLogger::new(
        LevelFilter::Info,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    CombinedLogger::init(vec![file_logger, term_logger])?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    innit_logging()?;

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
                    }
                    ServerEvent::ClientDisconnected { client_id, reason } => {
                        log::info!(
                            "[Dedicated Server] Client disconnected: {}, {}",
                            client_id,
                            reason
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
