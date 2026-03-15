use crate::game::ecs::systems::movement::*;
use crate::game::ecs::systems::snapshot::*;
use crate::game::server_world::*;
use crate::net::config::*;
use crate::net::protocol::snapshot::*;

pub struct ServerSim {
    server_world: ServerWorld,
    tick: u64,
    fixed_dt: f32,
}

impl Default for ServerSim {
    fn default() -> Self {
        Self {
            server_world: ServerWorld::default(),
            tick: 0,
            fixed_dt: FIXED_DT,
        }
    }
}

impl ServerSim {
    pub fn fixed_dt(&self) -> f32 {
        self.fixed_dt
    }

    pub fn reset(&mut self) {
        self.tick = 0;
        self.server_world.reset();

        // Spawn demo entities
        self.server_world
            .spawn_demo_entity(100.0, 100.0, 10.0, 10.0);
        self.server_world
            .spawn_demo_entity(500.0, 500.0, -10.0, -10.0);
    }

    pub fn step(&mut self) -> ServerWorldSnapshot {
        self.tick += 1;

        movement(self.server_world.world(), self.fixed_dt);

        let entity_positions: Vec<EntityPosition> =
            get_entity_position_data(self.server_world.world())
                .into_iter()
                .map(|(id, x, y)| EntityPosition { id, x, y })
                .collect();

        ServerWorldSnapshot::new(self.tick, entity_positions)
    }
}
