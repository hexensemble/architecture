use crate::game::ecs::components::movement::*;
use hecs::World;

#[derive(Debug, Clone, Copy)]
pub struct NetId(pub u32);

pub struct ServerWorld {
    world: World,
    next_net_id: NetId,
}

impl Default for ServerWorld {
    fn default() -> Self {
        Self {
            world: World::new(),
            next_net_id: NetId(1),
        }
    }
}

impl ServerWorld {
    pub fn reset(&mut self) {
        self.world.clear();
        self.next_net_id = NetId(1);
    }

    pub fn spawn_demo_entity(&mut self, x: f32, y: f32, vel_x: f32, vel_y: f32) -> u32 {
        let id = self.next_net_id.0;
        self.next_net_id = NetId(id + 1);

        self.world.spawn((
            NetId(id),
            Position { x, y },
            Velocity { x: vel_x, y: vel_y },
        ));

        id
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
