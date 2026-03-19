use crate::core::settings::*;
use crate::net::config::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::server_sim::ServerSim;
use crate::net::stepper::*;
use bitcode::{decode, encode};
use renet::{ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent};
use renet_netcode::{
    ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
    ServerConfig,
};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub trait GameSession {
    fn connect(&mut self);
    fn disconnect(&mut self);
    fn update(&mut self, frame_dt: f32);
    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot>;
}

pub fn make_session(settings: &NetSettings) -> Box<dyn GameSession> {
    match settings.mode {
        NetMode::Local => Box::new(LocalSession::default()),
        NetMode::Remote => Box::new(RemoteSession::new(settings.server_addr.clone())),
    }
}

pub struct LocalSession {
    server: Option<RenetServer>,
    server_transport: Option<NetcodeServerTransport>,

    client: Option<RenetClient>,
    client_transport: Option<NetcodeClientTransport>,

    stepper: FixedStepper,
    sim: ServerSim,
    latest_snapshot: Option<ServerWorldSnapshot>,
}

impl Default for LocalSession {
    fn default() -> Self {
        Self {
            server: None,
            server_transport: None,
            client: None,
            client_transport: None,
            stepper: FixedStepper::new(MAX_STEPS_PER_FRAME),
            sim: ServerSim::default(),
            latest_snapshot: None,
        }
    }
}

impl GameSession for LocalSession {
    fn connect(&mut self) {
        if self.server.is_some() {
            return;
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock before Unix epoch?");

        // Create local server

        let server = RenetServer::new(ConnectionConfig::default());

        let server_socket =
            UdpSocket::bind(LOCAL_ADDR).expect("Failed to bind local server UDP socket.");
        server_socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking on local server socket.");

        let server_addr = server_socket
            .local_addr()
            .expect("Local server address failed.");

        let server_config = ServerConfig {
            current_time,
            max_clients: MAX_CLIENTS,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![server_addr],
            authentication: ServerAuthentication::Unsecure,
        };

        let server_transport = NetcodeServerTransport::new(server_config, server_socket)
            .expect("Failed to initialize local server transport.");

        // Create client

        let client = RenetClient::new(ConnectionConfig::default());

        let client_socket = UdpSocket::bind(LOCAL_ADDR).expect("Failed to bind client UDP socket.");
        client_socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking on client socket.");

        let authentication = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id: make_client_id(),
            server_addr,
            user_data: None,
        };

        let client_transport =
            NetcodeClientTransport::new(current_time, authentication, client_socket)
                .expect("Failed to initialize client transport.");

        // Start local session

        self.sim.reset();

        self.server = Some(server);
        self.server_transport = Some(server_transport);

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;

        println!("[Local Server] Listening on: {}", server_addr);
    }

    fn disconnect(&mut self) {
        disconnect_client(
            &mut self.client,
            &mut self.client_transport,
            &mut self.latest_snapshot,
        );
    }

    fn update(&mut self, frame_dt: f32) {
        let (Some(server), Some(server_transport), Some(client), Some(client_transport)) = (
            self.server.as_mut(),
            self.server_transport.as_mut(),
            self.client.as_mut(),
            self.client_transport.as_mut(),
        ) else {
            return;
        };

        self.stepper.add_time(frame_dt.max(0.0));
        let fixed_dt = self.sim.fixed_dt();

        let mut should_disconnect = false;

        self.stepper.run_steps(fixed_dt, || {
            let dt = Duration::from_secs_f32(fixed_dt);

            // Update local server

            server.update(dt);

            if let Err(e) = server_transport.update(dt, server) {
                eprintln!("[Local Server] Server transport update error: {}", e);
                should_disconnect = true;
                return;
            }

            while let Some(event) = server.get_event() {
                match event {
                    ServerEvent::ClientConnected { client_id } => {
                        println!("[Local Server] Client connected: {}", client_id);
                        self.sim.reset();
                    }
                    ServerEvent::ClientDisconnected { client_id, reason } => {
                        println!(
                            "[Local Server] Client disconnected: {}, {}",
                            client_id, reason
                        );
                    }
                }
            }

            let any_clients = server.clients_id_iter().next().is_some();
            if any_clients {
                let snapshot = self.sim.step();
                let msg = ServerMessage::Snapshot(snapshot);
                server.broadcast_message(DefaultChannel::Unreliable, encode(&msg));
            }

            server_transport.send_packets(server);

            // Update client

            client.update(dt);

            if let Err(e) = client_transport.update(dt, client) {
                eprintln!("[Client] Client transport update error: {}", e);
                should_disconnect = true;
                return;
            }

            while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
                if let Ok(ServerMessage::Snapshot(snapshot)) = decode(bytes.as_ref()) {
                    self.latest_snapshot = Some(snapshot);
                }
            }

            if let Err(e) = client_transport.send_packets(client) {
                eprintln!("[Client] Send packets error: {}", e);
            }
        });

        if should_disconnect {
            self.disconnect();
        }
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.latest_snapshot.as_ref()
    }
}

pub struct RemoteSession {
    server_addr: SocketAddr,

    client: Option<RenetClient>,
    client_transport: Option<NetcodeClientTransport>,

    latest_snapshot: Option<ServerWorldSnapshot>,
}

impl RemoteSession {
    pub fn new(server_addr: String) -> Self {
        if let Ok(server_addr) = server_addr.parse() {
            Self {
                server_addr,
                client: None,
                client_transport: None,
                latest_snapshot: None,
            }
        } else {
            panic!("Invalid server address! Must be in format: 127.0.0.1:27960");
        }
    }
}

impl GameSession for RemoteSession {
    fn connect(&mut self) {
        if self.client.is_some() {
            return;
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock before Unix epoch?");

        // Create client

        let client = RenetClient::new(ConnectionConfig::default());

        let client_socket =
            UdpSocket::bind(CLIENT_ADDR).expect("Failed to bind client UPD socket.");
        client_socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking on client socket.");

        let authentication = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id: make_client_id(),
            server_addr: self.server_addr,
            user_data: None,
        };

        let client_transport =
            NetcodeClientTransport::new(current_time, authentication, client_socket)
                .expect("Failed to initialize client transport.");

        // Start remote session

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;
    }

    fn disconnect(&mut self) {
        disconnect_client(
            &mut self.client,
            &mut self.client_transport,
            &mut self.latest_snapshot,
        );
    }

    fn update(&mut self, frame_dt: f32) {
        let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        else {
            return;
        };

        let dt = Duration::from_secs_f32(frame_dt.max(0.0));

        // Update client

        client.update(dt);

        if let Err(e) = client_transport.update(dt, client) {
            eprintln!("[Client] Client transport update error: {}", e);
            client.disconnect();
            return;
        }

        while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
            if let Ok(ServerMessage::Snapshot(snapshot)) = decode(bytes.as_ref()) {
                self.latest_snapshot = Some(snapshot);
            }
        }

        if let Err(e) = client_transport.send_packets(client) {
            eprintln!("[Client] Send packets error: {}", e);
        }
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.latest_snapshot.as_ref()
    }
}

fn disconnect_client(
    client: &mut Option<RenetClient>,
    client_transport: &mut Option<NetcodeClientTransport>,
    latest_snapshot: &mut Option<ServerWorldSnapshot>,
) {
    if let (Some(c), Some(ct)) = (client.as_mut(), client_transport.as_mut()) {
        c.disconnect();

        // Flush packets
        let _ = ct.send_packets(c);
    }

    *client = None;
    *client_transport = None;

    *latest_snapshot = None;
}
