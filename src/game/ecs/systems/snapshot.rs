use crate::game::ecs::components::movement::*;
use crate::game::server_world::*;
use hecs::World;

pub fn get_entity_position_data(world: &World) -> Vec<(u32, f32, f32)> {
    let mut entity_positions = Vec::new();

    for (_, (net_id, pos)) in world.query::<(&NetId, &Position)>().iter() {
        entity_positions.push((net_id.0, pos.x, pos.y));
    }

    entity_positions
}
