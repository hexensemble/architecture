use crate::game::ecs::components::movement::*;
use crate::game::ecs::components::player::*;
use hecs::Entity;
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

    pub fn spawn_player(&mut self, client_id: u64) -> u32 {
        let id = self.next_net_id.0;
        self.next_net_id = NetId(id + 1);

        self.world.spawn((
            NetId(id),
            ClientOwner(client_id),
            Position { x: 400.0, y: 300.0 },
            Velocity { x: 0.0, y: 0.0 },
        ));

        id
    }

    pub fn spawn_enemy(&mut self, x: f32, y: f32, vel_x: f32, vel_y: f32) -> u32 {
        let id = self.next_net_id.0;
        self.next_net_id = NetId(id + 1);

        self.world.spawn((
            NetId(id),
            Position { x, y },
            Velocity { x: vel_x, y: vel_y },
        ));

        id
    }

    pub fn despawn_player(&mut self, client_id: u64) {
        let mut to_despawn = None;

        for (entity, owner) in self.world.query::<(Entity, &ClientOwner)>().iter() {
            if owner.0 == client_id {
                to_despawn = Some(entity);
                break;
            }
        }

        if let Some(entity) = to_despawn {
            let _ = self.world.despawn(entity);
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn mut_world(&mut self) -> &mut World {
        &mut self.world
    }
}
