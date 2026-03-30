use uuid::Uuid;

pub const PROTOCOL_ID: u64 = 0x11223344;
pub const MAX_STEPS_PER_FRAME: u32 = 8;
pub const MAX_CLIENTS: usize = 64;
pub const LOCAL_ADDR: &str = "127.0.0.1:0";
pub const CLIENT_ADDR: &str = "0.0.0.0:0";
pub const FIXED_DT: f32 = 1.0 / 60.0;

pub fn make_client_id() -> u64 {
    let id = Uuid::new_v4();

    id.as_u128() as u64
}
