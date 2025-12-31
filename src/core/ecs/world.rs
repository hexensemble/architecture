use crate::core::ecs::resources::*;
use hecs::World;

pub struct EcsWorld {
    world: World,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn run_system<F>(&mut self, resources: &EcsResources, mut f: F)
    where
        F: FnMut(&EcsResources, &mut World),
    {
        f(resources, self.world_mut())
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
