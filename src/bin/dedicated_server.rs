use architecture::game::ecs::systems::movement::*;
use architecture::game::ecs::systems::snapshot::*;
use architecture::game::world::ServerWorld;
use architecture::net::protocol::message::ServerMessage;
use architecture::net::protocol::snapshot::*;
use architecture::net::renet_config::PROTOCOL_ID;
use bitcode::encode;
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use std::env;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:27960".to_string());

    let server_addr = addr.parse()?;

    let socket = UdpSocket::bind(server_addr)?;
    socket.set_nonblocking(true)?;

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let mut server = RenetServer::new(ConnectionConfig::default());

    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let mut transport = NetcodeServerTransport::new(server_config, socket)?;

    let mut server_world = ServerWorld::default();
    let mut tick: u64 = 0;

    server_world.reset();
    server_world.spawn_demo_entity(100.0, 100.0, 10.0, 10.0);
    server_world.spawn_demo_entity(500.0, 500.0, -10.0, -10.0);

    let dt = Duration::from_secs_f32(1.0 / 60.0);

    println!("[Dedicated Server] Listening on: {}.", server_addr);

    loop {
        server.update(dt);
        transport.update(dt, &mut server)?;

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("[Dedicated Server] Client connected: {}.", client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!(
                        "[Dedicated Server] Client disconnected: {}. Reason: {}.",
                        client_id, reason
                    );
                }
            }
        }

        tick += 1;

        movement(server_world.world(), dt.as_secs_f32());

        let entity_positions: Vec<EntityPosition> = get_entity_position_data(server_world.world())
            .into_iter()
            .map(|(id, x, y)| EntityPosition { id, x, y })
            .collect();

        let any_clients = server.clients_id_iter().next().is_some();
        if any_clients {
            let snapshot = ServerWorldSnapshot::new(tick, entity_positions);
            let msg = ServerMessage::Snapshot(snapshot);

            server.broadcast_message(DefaultChannel::Unreliable, encode(&msg));
        }

        transport.send_packets(&mut server);

        std::thread::sleep(dt);
    }
}
