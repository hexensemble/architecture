use crate::core::action::*;
use crate::core::ecs::resources::*;
use hecs::World;
use raylib::prelude::*;

pub struct EcsWorld {
    world: World,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn run_system<A, F>(&mut self, resources: &EcsResources<A>, mut f: F)
    where
        A: ActionType,
        F: FnMut(&EcsResources<A>, &mut World),
    {
        f(resources, &mut self.world)
    }

    pub fn run_render_system<F>(&self, d: &mut RaylibDrawHandle, f: F)
    where
        F: Fn(&mut RaylibDrawHandle, &World),
    {
        f(d, &self.world);
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
