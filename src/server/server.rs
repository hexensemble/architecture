use crate::core::protocol::message::*;
use crate::core::protocol::snapshot::*;
use crate::core::transport::endpoint::*;

pub struct Server<T>
where
    T: ServerEndpoint,
{
    endpoint: T,
    tick: u64,
    client_connected: bool,
}

impl<T: ServerEndpoint> Server<T> {
    pub fn new(endpoint: T) -> Self {
        Self {
            endpoint,
            tick: 0,
            client_connected: false,
        }
    }

    pub fn tick(&mut self) {
        while let Some(msg) = self.endpoint.recv() {
            match msg {
                ClientMessage::Connect => {
                    println!("[Server] Client connected.");
                    self.client_connected = true;
                    let _ = self
                        .endpoint
                        .send(ServerMessage::Connected { client_id: 1 });
                }
                ClientMessage::Disconnect => {
                    println!("[Server] Client disconnected.");
                    self.client_connected = false;
                    let _ = self.endpoint.send(ServerMessage::Disconnected);
                }
            }
        }

        if self.client_connected {
            self.tick += 1;
            let snapshot = WorldSnapshot { tick: self.tick };
            let _ = self.endpoint.send(ServerMessage::Snapshot(snapshot));
        }
    }

    pub fn is_client_connected(&self) -> bool {
        self.client_connected
    }

    pub fn current_tick(&self) -> u64 {
        self.tick
    }
}
