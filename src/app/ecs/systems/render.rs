use crate::app::ecs::components::movement::*;
use hecs::World;
use raylib::prelude::*;

pub fn draw_positions(d: &mut RaylibDrawHandle, world: &World) {
    for (_, pos) in world.query::<&Position>().iter() {
        d.draw_circle(pos.x as i32, pos.y as i32, 10.0, Color::BLUE);
    }
}
