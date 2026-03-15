use std::time::{SystemTime, UNIX_EPOCH};

pub const PROTOCOL_ID: u64 = 0x11223344;
pub const MAX_STEPS_PER_FRAME: u32 = 8;
pub const MAX_CLIENTS: usize = 64;
pub const LOCAL_ADDR: &str = "127.0.0.1:0";
pub const CLIENT_ADDR: &str = "0.0.0.0:0";
pub const FIXED_DT: f32 = 1.0 / 60.0;

pub fn make_client_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
