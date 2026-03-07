use crate::core::settings::*;
use crate::net::client::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::renet_config::*;
use crate::net::server::*;
use crate::net::stepper::*;
use crate::net::transport::loopback::*;
use bitcode::decode;
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};
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
    client: Client<LoopBackClientEndpoint>,
    server: Server<LoopBackServerEndpoint>,
    stepper: FixedStepper,
}

impl Default for LocalSession {
    fn default() -> Self {
        let (client_endpoint, server_endpoint) = loopback();

        Self {
            client: Client::new(client_endpoint),
            server: Server::new(server_endpoint),
            stepper: FixedStepper::new(8),
        }
    }
}

impl GameSession for LocalSession {
    fn connect(&mut self) {
        let _ = self.client.connect();
        self.server.poll_messages();
        self.client.poll();
    }

    fn disconnect(&mut self) {
        let _ = self.client.disconnect();
        self.server.poll_messages();
        self.client.poll();
    }

    fn update(&mut self, frame_dt: f32) {
        self.client.poll();
        self.server.poll_messages();

        let fixed_dt = self.server.fixed_dt();
        self.stepper.add_time(frame_dt);

        self.stepper.run_steps(fixed_dt, || {
            self.server.step();
            self.client.poll();
            self.server.poll_messages();
        });
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.client.server_world_snapshot().as_ref()
    }
}

pub struct RemoteSession {
    server_addr: SocketAddr,
    client: Option<RenetClient>,
    transport: Option<NetcodeClientTransport>,
    latest_snapshot: Option<ServerWorldSnapshot>,
}

impl RemoteSession {
    pub fn new(server_addr: String) -> Self {
        let server_addr = server_addr
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:27960".parse().unwrap());

        Self {
            server_addr,
            client: None,
            transport: None,
            latest_snapshot: None,
        }
    }
}

impl GameSession for RemoteSession {
    fn connect(&mut self) {
        if self.client.is_some() {
            return;
        }

        let client = RenetClient::new(ConnectionConfig::default());

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock before Unix epoch?");

        let authentication = ClientAuthentication::Unsecure {
            server_addr: self.server_addr,
            client_id: make_client_id(),
            user_data: None,
            protocol_id: PROTOCOL_ID,
        };

        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind client UPD socket.");
        let _ = socket.set_nonblocking(true);

        let transport = NetcodeClientTransport::new(current_time, authentication, socket)
            .expect("Netcode init.");

        self.client = Some(client);
        self.transport = Some(transport);
        self.latest_snapshot = None;
    }

    fn disconnect(&mut self) {
        if let Some(client) = self.client.as_mut() {
            client.disconnect();
        }

        self.client = None;
        self.transport = None;
        self.latest_snapshot = None;
    }

    fn update(&mut self, frame_dt: f32) {
        let (Some(client), Some(transport)) = (self.client.as_mut(), self.transport.as_mut())
        else {
            return;
        };

        let dt = Duration::from_secs_f32(frame_dt.max(0.0));

        client.update(dt);

        if let Err(e) = transport.update(dt, client) {
            eprintln!("[Remote Session] Transport update error: {}", e);
            transport.disconnect_reason();

            self.client = None;
            self.transport = None;
            self.latest_snapshot = None;

            return;
        }

        while let Some(bytes) = client.receive_message(DefaultChannel::Unreliable) {
            match decode::<ServerMessage>(bytes.as_ref()) {
                Ok(ServerMessage::Snapshot(snapshot)) => self.latest_snapshot = Some(snapshot),
                Ok(_) => {}
                Err(_) => {}
            }
        }

        if let Err(e) = transport.send_packets(client) {
            eprintln!("[Remote Session] Send packets error: {}", e);
        }
    }

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        self.latest_snapshot.as_ref()
    }
}
