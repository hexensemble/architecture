use crate::game::ecs::components::movement::*;
use crate::game::ecs::components::player::*;
use crate::game::ecs::systems::movement::*;
use crate::game::ecs::systems::snapshot::*;
use crate::game::server_world::*;
use crate::net::config::*;
use crate::net::protocol::input::*;
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

    pub fn spawn_player(&mut self, client_id: u64) {
        self.server_world.spawn_player(client_id);
    }

    pub fn despawn_player(&mut self, client_id: u64) {
        self.server_world.despawn_player(client_id);
    }

    pub fn handle_input(&mut self, client_id: u64, input: PlayerInput) {
        for (_, (owner, vel)) in self
            .server_world
            .world()
            .query::<(&ClientOwner, &mut Velocity)>()
            .iter()
        {
            if owner.0 == client_id {
                match input {
                    PlayerInput::Up => vel.y -= PLAYER_SPEED,
                    PlayerInput::Down => vel.y = PLAYER_SPEED,
                    PlayerInput::Left => vel.x -= PLAYER_SPEED,
                    PlayerInput::Right => vel.x = PLAYER_SPEED,
                }

                break;
            }
        }
    }

    pub fn reset_player_velocities(&mut self) {
        for (_, (_, vel)) in self
            .server_world
            .world()
            .query::<(&ClientOwner, &mut Velocity)>()
            .iter()
        {
            vel.x = 0.0;
            vel.y = 0.0;
        }
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
