use crate::game::ecs::components::movement::*;
use hecs::World;

pub fn movement(world: &World, dt: f32) {
    for (id, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
