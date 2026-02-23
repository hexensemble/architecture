use crate::net::protocol::snapshot::*;

#[derive(Debug, Clone)]
pub enum ClientMessage {
    Connect,
    Disconnect,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Connected { client_id: u32 },
    Disconnected,
    Snapshot(ServerWorldSnapshot),
}
