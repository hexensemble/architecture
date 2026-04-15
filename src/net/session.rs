use crate::engine::settings::*;
use crate::net::config::*;
use crate::net::protocol::input::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::server_sim::*;
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
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn disconnect(&mut self);
    fn update(&mut self, frame_dt: f32);
    fn send_input(&mut self, input: PlayerInput);
    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot>;
}

pub fn make_session(
    settings: &NetSettings,
) -> Result<Box<dyn GameSession>, Box<dyn std::error::Error>> {
    match settings.mode {
        NetMode::Local => Ok(Box::new(LocalSession::default())),
        NetMode::Remote => {
            let remote_session = RemoteSession::new(settings.server_addr.clone())?;

            Ok(Box::new(remote_session))
        }
    }
}

pub struct LocalSession {
    server: Option<RenetServer>,
    server_transport: Option<NetcodeServerTransport>,

    client: Option<RenetClient>,
    client_transport: Option<NetcodeClientTransport>,
    client_connected: bool,

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
            client_connected: false,
            stepper: FixedStepper::new(MAX_STEPS_PER_FRAME),
            sim: ServerSim::default(),
            latest_snapshot: None,
        }
    }
}

impl GameSession for LocalSession {
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.server.is_some() {
            return Err("Server already exists!".into());
        }

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

        // Create local server

        let server = RenetServer::new(ConnectionConfig::default());

        let server_socket = UdpSocket::bind(LOCAL_ADDR)?;
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

        // Create client

        let client = RenetClient::new(ConnectionConfig::default());

        let client_socket = UdpSocket::bind(LOCAL_ADDR)?;
        client_socket.set_nonblocking(true)?;

        let authentication = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id: make_client_id(),
            server_addr,
            user_data: None,
        };

        let client_transport =
            NetcodeClientTransport::new(current_time, authentication, client_socket)?;

        // Start local session

        log::info!("[Local Server] Starting server...");

        self.sim.reset();

        self.server = Some(server);
        self.server_transport = Some(server_transport);

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;

        log::info!("[Local Server] Listening on: {}", server_addr);
        log::info!("[Client] Connecting to server on: {}", server_addr);

        Ok(())
    }

    fn disconnect(&mut self) {
        // Disconnect Client

        if let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        {
            client.disconnect();

            // Flush packets
            let _ = client_transport.send_packets(client);

            log::info!("[Client] Disconnected from server")
        }

        // Disconnect Server

        if let (Some(server), Some(server_transport)) =
            (self.server.as_mut(), self.server_transport.as_mut())
        {
            server_transport.disconnect_all(server);

            while let Some(event) = server.get_event() {
                match event {
                    ServerEvent::ClientDisconnected { client_id, reason } => {
                        log::info!(
                            "[Local Server] Client disconnected: {}, {}",
                            client_id,
                            reason
                        );

                        self.sim.despawn_player(client_id);
                    }
                    _ => {}
                }
            }
        }

        self.client = None;
        self.client_transport = None;

        self.server = None;
        self.server_transport = None;

        self.latest_snapshot = None;
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

        if client.is_connected() && !self.client_connected {
            self.client_connected = true;
            log::info!("[Client] Connected to server");
        }

        self.stepper.add_time(frame_dt.max(0.0));
        let fixed_dt = self.sim.fixed_dt();

        let mut should_disconnect = false;

        self.stepper.run_steps(fixed_dt, || {
            let dt = Duration::from_secs_f32(fixed_dt);

            // Update local server

            server.update(dt);

            if let Err(e) = server_transport.update(dt, server) {
                log::error!("[Local Server] Server transport update error: {}", e);
                should_disconnect = true;
                return;
            }

            while let Some(event) = server.get_event() {
                match event {
                    ServerEvent::ClientConnected { client_id } => {
                        log::info!("[Local Server] Client connected: {}", client_id);
                        self.sim.reset();

                        self.sim.spawn_player(client_id);
                    }
                    _ => {}
                }
            }

            // Clear old client inputs
            self.sim.reset_player_velocities();
            // Process new client inputs
            let client_ids: Vec<u64> = server.clients_id_iter().collect();
            for client_id in client_ids {
                while let Some(bytes) =
                    server.receive_message(client_id, DefaultChannel::ReliableOrdered)
                {
                    if let Ok(ClientMessage::Input(input)) = decode(bytes.as_ref()) {
                        self.sim.handle_input(client_id, input);
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
                log::error!("[Client] Client transport update error: {}", e);
                should_disconnect = true;
                return;
            }

            while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
                if let Ok(ServerMessage::Snapshot(snapshot)) = decode(bytes.as_ref()) {
                    self.latest_snapshot = Some(snapshot);
                }
            }

            if let Err(e) = client_transport.send_packets(client) {
                log::error!("[Client] Send packets error: {}", e);
            }
        });

        if should_disconnect {
            self.disconnect();
        }
    }

    fn send_input(&mut self, input: PlayerInput) {
        send_client_input(&mut self.client, input);
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.latest_snapshot.as_ref()
    }
}

pub struct RemoteSession {
    server_addr: SocketAddr,

    client: Option<RenetClient>,
    client_transport: Option<NetcodeClientTransport>,
    client_connected: bool,

    stepper: FixedStepper,
    latest_snapshot: Option<ServerWorldSnapshot>,
}

impl RemoteSession {
    pub fn new(server_addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let parsed_addr = server_addr.parse()?;

        Ok(Self {
            server_addr: parsed_addr,
            client: None,
            client_transport: None,
            client_connected: false,
            stepper: FixedStepper::new(MAX_STEPS_PER_FRAME),
            latest_snapshot: None,
        })
    }
}

impl GameSession for RemoteSession {
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.client.is_some() {
            return Err("Client already exists!".into());
        }

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

        // Create client

        let client = RenetClient::new(ConnectionConfig::default());

        let client_socket = UdpSocket::bind(CLIENT_ADDR)?;
        client_socket.set_nonblocking(true)?;

        let authentication = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id: make_client_id(),
            server_addr: self.server_addr,
            user_data: None,
        };

        let client_transport =
            NetcodeClientTransport::new(current_time, authentication, client_socket)?;

        // Start remote session

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;

        log::info!("[Client] Connecting to server on: {}", self.server_addr);

        Ok(())
    }

    fn disconnect(&mut self) {
        // Disconnect Client

        if let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        {
            client.disconnect();

            // Flush packets
            let _ = client_transport.send_packets(client);

            log::info!("[Client] Disconnected from server")
        }

        self.client = None;
        self.client_transport = None;

        self.latest_snapshot = None;
    }

    fn update(&mut self, frame_dt: f32) {
        let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        else {
            return;
        };

        if client.is_connected() && !self.client_connected {
            self.client_connected = true;
            log::info!("[Client] Connected to server")
        }

        self.stepper.add_time(frame_dt.max(0.0));
        let fixed_dt = FIXED_DT;

        let mut should_disconnect = false;

        self.stepper.run_steps(fixed_dt, || {
            let dt = Duration::from_secs_f32(fixed_dt);

            // Update client

            client.update(dt);

            if let Err(e) = client_transport.update(dt, client) {
                log::error!("[Client] Client transport update error: {}", e);
                should_disconnect = true;
                return;
            }

            while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
                if let Ok(ServerMessage::Snapshot(snapshot)) = decode(bytes.as_ref()) {
                    self.latest_snapshot = Some(snapshot);
                }
            }

            if let Err(e) = client_transport.send_packets(client) {
                log::error!("[Client] Send packets error: {}", e);
            }
        });

        if should_disconnect {
            self.disconnect();
        }
    }

    fn send_input(&mut self, input: PlayerInput) {
        send_client_input(&mut self.client, input);
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.latest_snapshot.as_ref()
    }
}

fn send_client_input(client: &mut Option<RenetClient>, input: PlayerInput) {
    if let Some(c) = client.as_mut() {
        let msg = ClientMessage::Input(input);

        c.send_message(DefaultChannel::ReliableOrdered, encode(&msg));
    }
}
