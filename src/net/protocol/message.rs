use crate::net::protocol::snapshot::*;
use bitcode::{Decode, Encode};

#[derive(Clone, Debug, Decode, Encode)]
pub enum ServerMessage {
    Snapshot(ServerWorldSnapshot),
}
