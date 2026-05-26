use crate::game::ecs::components::movement::*;
use hecs::World;

pub fn movement(world: &World, dt: f32) {
    for (pos, vel) in world.query::<(&mut Position, &Velocity)>().iter() {
        pos.x = (pos.x + vel.x * dt).clamp(0.0, 800.0);
        pos.y = (pos.y + vel.y * dt).clamp(0.0, 600.0);
    }
}
