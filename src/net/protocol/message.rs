use crate::net::protocol::snapshot::*;
use bitcode::{Decode, Encode};

#[derive(Clone, Debug, Decode, Encode)]
pub enum ClientMessage {
    Connect,
    Disconnect,
}

#[derive(Clone, Debug, Decode, Encode)]
pub enum ServerMessage {
    Connected { client_id: u32 },
    Disconnected,
    Snapshot(ServerWorldSnapshot),
}
