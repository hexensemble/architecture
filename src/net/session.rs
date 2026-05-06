use crate::engine::settings::*;
use crate::net::config::*;
use crate::net::protocol::input::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::server_sim::*;
use crate::net::stepper::*;
use crate::net::transport::*;
use bitcode::encode;
use renet::{DefaultChannel, RenetClient, RenetServer, ServerEvent};
use renet_netcode::{NetcodeClientTransport, NetcodeServerTransport};
use std::net::SocketAddr;

pub trait GameSession {
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn update(&mut self, frame_dt: f32);
    fn disconnect(&mut self);
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

        // Create local server

        let (server, server_transport, server_addr) = create_server(LOCAL_ADDR.to_string())?;

        // Create client

        let (client, client_transport) = create_client(LOCAL_ADDR.to_string(), server_addr)?;

        // Start local session

        log::info!("[Local Server] Starting server");

        self.sim.reset();

        self.server = Some(server);
        self.server_transport = Some(server_transport);

        log::info!("[Local Server] Listening on: {}", server_addr);

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;

        log::info!("[Client] Connecting to server on: {}", server_addr);

        Ok(())
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
            update_server(
                "Local Server".into(),
                server,
                server_transport,
                &mut self.sim,
                fixed_dt,
            );

            update_client(
                client,
                client_transport,
                &mut self.latest_snapshot,
                &mut should_disconnect,
                fixed_dt,
            );
        });

        if should_disconnect {
            self.disconnect();
        }
    }

    fn disconnect(&mut self) {
        // Disconnect Client

        if let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        {
            disconnect_client(client, client_transport);
        }

        // Disconnect Server

        if let (Some(server), Some(server_transport)) =
            (self.server.as_mut(), self.server_transport.as_mut())
        {
            server_transport.disconnect_all(server);

            while let Some(event) = server.get_event() {
                if let ServerEvent::ClientDisconnected { client_id, reason } = event {
                    log::info!(
                        "[Local Server] Client disconnected: {}, {}",
                        client_id,
                        reason
                    );

                    self.sim.despawn_player(client_id);
                }
            }

            log::info!("[Local Server] Shutting down server");

            // Flush packets
            server_transport.send_packets(server);
            log::info!("[Local Server] Shutdown complete");
        }

        self.client = None;
        self.client_transport = None;

        self.server = None;
        self.server_transport = None;

        self.latest_snapshot = None;
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

        // Create client

        let (client, client_transport) = create_client(CLIENT_ADDR.to_string(), self.server_addr)?;

        // Start remote session

        self.client = Some(client);
        self.client_transport = Some(client_transport);

        self.latest_snapshot = None;

        log::info!("[Client] Connecting to server on: {}", self.server_addr);

        Ok(())
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
            update_client(
                client,
                client_transport,
                &mut self.latest_snapshot,
                &mut should_disconnect,
                fixed_dt,
            );
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

    fn disconnect(&mut self) {
        // Disconnect Client

        if let (Some(client), Some(client_transport)) =
            (self.client.as_mut(), self.client_transport.as_mut())
        {
            disconnect_client(client, client_transport);
        }

        self.client = None;
        self.client_transport = None;

        self.latest_snapshot = None;
    }
}

fn send_client_input(client: &mut Option<RenetClient>, input: PlayerInput) {
    if let Some(c) = client.as_mut() {
        let msg = ClientMessage::Input(input);

        c.send_message(DefaultChannel::ReliableOrdered, encode(&msg));
    }
}
