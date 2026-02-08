use crate::core::protocol::snapshot::*;

/// Messages sent from client to server
#[derive(Debug, Clone)]
pub enum ClientMessage {
    Connect,
    Disconnect,
}

/// Messages sent from server to client
#[derive(Debug, Clone)]
pub enum ServerMessage {
    Connected { client_id: u32 },
    Disconnected,
    Snapshot(WorldSnapshot),
}
