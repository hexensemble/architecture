#[derive(Clone, Debug)]
pub struct ServerWorldSnapshot {
    tick: u64,
    entity_positions: Vec<EntityPosition>,
}

impl Default for ServerWorldSnapshot {
    fn default() -> Self {
        Self {
            tick: 666,
            entity_positions: Vec::new(),
        }
    }
}

impl ServerWorldSnapshot {
    pub fn new(tick: u64, entity_positions: Vec<EntityPosition>) -> Self {
        Self {
            tick,
            entity_positions,
        }
    }

    pub fn snapshot_tick(&self) -> u64 {
        self.tick
    }

    pub fn entity_positions(&self) -> &Vec<EntityPosition> {
        &self.entity_positions
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EntityPosition {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}
