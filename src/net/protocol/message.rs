use crate::net::protocol::input::*;
use crate::net::protocol::snapshot::*;
use bitcode::{Decode, Encode};

#[derive(Clone, Debug, Decode, Encode)]
pub enum ServerMessage {
    Snapshot(ServerWorldSnapshot),
}

#[derive(Clone, Debug, Decode, Encode)]
pub enum ClientMessage {
    Input(PlayerInput),
}
