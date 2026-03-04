use crate::core::settings::*;
use crate::net::client::*;
use crate::net::protocol::snapshot::*;
use crate::net::server::*;
use crate::net::stepper::*;
use crate::net::transport::loopback::*;

pub trait GameSession {
    fn connect(&mut self);
    fn disconnect(&mut self);
    fn update(&mut self, frame_dt: f32);
    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot>;
}

pub fn make_session(settings: &NetSettings) -> Box<dyn GameSession> {
    match settings.mode {
        NetMode::Local => Box::new(LocalSession::default()),
        NetMode::Remote => Box::new(RemoteSession::default()),
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

//TODO
pub struct RemoteSession {}

//TODO
impl Default for RemoteSession {
    fn default() -> Self {
        Self {}
    }
}

//TODO
impl GameSession for RemoteSession {
    fn connect(&mut self) {}

    fn disconnect(&mut self) {}

    fn update(&mut self, frame_dt: f32) {}

    fn latest_snapshot(&self) -> Option<&ServerWorldSnapshot> {
        None
    }
}
