use crate::app::ecs::components::movement::*;
use crate::core::ecs::resources::*;
use hecs::World;

pub fn movement(resources: &EcsResources, world: &mut World) {
    for (id, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
        pos.x += vel.x * resources.time.delta();
        pos.y += vel.y * resources.time.delta();

        println!("Entity {:?}, {:?}, {:?}", id, pos, vel);
    }
}
