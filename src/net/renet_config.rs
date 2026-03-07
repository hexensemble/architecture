use std::time::{SystemTime, UNIX_EPOCH};

pub const PROTOCOL_ID: u64 = 0x11223344;

pub fn make_client_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
