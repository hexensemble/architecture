use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::transport::endpoint::*;
use crate::net::transport::error::*;

pub struct Client<T>
where
    T: ClientEndpoint,
{
    endpoint: T,
    server_world_snapshot: Option<ServerWorldSnapshot>,
}

impl<T: ClientEndpoint> Client<T> {
    pub fn new(endpoint: T) -> Self {
        Self {
            endpoint,
            server_world_snapshot: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), TransportError> {
        self.endpoint.send(ClientMessage::Connect)
    }

    pub fn disconnect(&mut self) -> Result<(), TransportError> {
        self.endpoint.send(ClientMessage::Disconnect)
    }

    pub fn poll(&mut self) {
        while let Some(msg) = self.endpoint.recv() {
            match msg {
                ServerMessage::Connected { .. } => {
                    println!("[Client] Server connected.")
                }
                ServerMessage::Disconnected => {
                    println!("[Client] Server disconnected.");
                    self.server_world_snapshot = None;
                }
                ServerMessage::Snapshot(snapshot) => self.server_world_snapshot = Some(snapshot),
            }
        }
    }

    pub fn mut_endpoint(&mut self) -> &mut T {
        &mut self.endpoint
    }

    pub fn server_world_snapshot(&self) -> &Option<ServerWorldSnapshot> {
        &self.server_world_snapshot
    }
}
