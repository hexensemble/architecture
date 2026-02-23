use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::transport::endpoint::*;

pub struct Client<T>
where
    T: ClientEndpoint,
{
    endpoint: T,
    accumulator: f32,
    server_world_snapshot: Option<ServerWorldSnapshot>,
}

impl<T: ClientEndpoint> Client<T> {
    pub fn new(endpoint: T) -> Self {
        Self {
            endpoint,
            accumulator: 0.0,
            server_world_snapshot: None,
        }
    }

    pub fn get_server_messages(&mut self) {
        while let Some(msg) = self.endpoint.recv() {
            match msg {
                ServerMessage::Connected { .. } => {}
                ServerMessage::Disconnected => self.server_world_snapshot = None,
                ServerMessage::Snapshot(snapshot) => self.server_world_snapshot = Some(snapshot),
            }
        }
    }

    pub fn mut_endpoint(&mut self) -> &mut T {
        &mut self.endpoint
    }

    pub fn get_accumulator(&self) -> f32 {
        self.accumulator
    }

    pub fn add_time(&mut self, dt: f32) {
        self.accumulator += dt;
    }

    pub fn subtract_time(&mut self, dt: f32) {
        self.accumulator -= dt;
    }

    pub fn server_world_snapshot(&self) -> &Option<ServerWorldSnapshot> {
        &self.server_world_snapshot
    }
}
