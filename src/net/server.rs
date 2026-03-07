use crate::game::ecs::systems::movement::*;
use crate::game::ecs::systems::snapshot::*;
use crate::game::world::*;
use crate::net::protocol::message::*;
use crate::net::protocol::snapshot::*;
use crate::net::transport::endpoint::*;

pub struct Server<T>
where
    T: ServerEndpoint,
{
    endpoint: T,
    fixed_dt: f32,
    tick: u64,
    client_connected: bool,
    server_world: ServerWorld,
}

impl<T: ServerEndpoint> Server<T> {
    pub fn new(endpoint: T) -> Self {
        Self {
            endpoint,
            fixed_dt: 1.0 / 60.0,
            tick: 0,
            client_connected: false,
            server_world: ServerWorld::default(),
        }
    }

    pub fn poll_messages(&mut self) {
        while let Some(msg) = self.endpoint.recv() {
            match msg {
                ClientMessage::Connect => {
                    println!("[Server] Client connected.");
                    self.client_connected = true;

                    self.tick = 0;
                    self.server_world.reset();
                    self.server_world
                        .spawn_demo_entity(100.0, 100.0, 10.0, 10.0);
                    self.server_world
                        .spawn_demo_entity(500.0, 500.0, -10.0, -10.0);

                    let _ = self
                        .endpoint
                        .send(ServerMessage::Connected { client_id: 1 });
                }
                ClientMessage::Disconnect => {
                    println!("[Server] Client disconnected.");
                    self.client_connected = false;
                    self.server_world.reset();

                    let _ = self.endpoint.send(ServerMessage::Disconnected);
                }
            }
        }
    }

    pub fn step(&mut self) {
        if !self.client_connected {
            return;
        }

        self.tick += 1;

        let dt = self.fixed_dt();
        movement(self.server_world.world(), dt);

        let entity_positions: Vec<EntityPosition> =
            get_entity_position_data(self.server_world.world())
                .into_iter()
                .map(|(id, x, y)| EntityPosition { id, x, y })
                .collect();

        let snapshot = ServerWorldSnapshot::new(self.tick, entity_positions);

        let _ = self.endpoint.send(ServerMessage::Snapshot(snapshot));
    }

    // Tests
    pub fn tick(&mut self) {
        self.poll_messages();
        self.step();
    }

    pub fn fixed_dt(&self) -> f32 {
        self.fixed_dt
    }

    // Tests
    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    // Tests
    pub fn client_connected(&self) -> bool {
        self.client_connected
    }
}
