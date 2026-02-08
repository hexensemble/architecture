/// World state snapshot sent to clients
#[derive(Debug, Clone, Default)]
pub struct WorldSnapshot {
    pub tick: u64,
}
