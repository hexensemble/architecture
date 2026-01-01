use crate::app::ecs::components::movement::*;
use crate::app::ecs::systems::movement::*;
use crate::app::ecs::systems::render::*;
use crate::app::layers::menu::MenuLayer;
use crate::app::layers::pause::PauseLayer;
use crate::core::action::*;
use crate::core::context::*;
use crate::core::ecs::resources::*;
use crate::core::ecs::world::*;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct GameLayer {
    ecs: EcsWorld,
}

impl GameLayer {
    pub fn new() -> Self {
        let mut ecs = EcsWorld::new();

        ecs.world_mut().spawn((
            Position { x: 100.0, y: 100.0 },
            Velocity { x: 10.0, y: 10.0 },
        ));

        Self { ecs }
    }
}

impl Layer for GameLayer {
    fn on_event(&mut self, ctx: &mut AppContext, event: &Event) {}

    fn on_update(&mut self, ctx: &mut AppContext, rl: &mut RaylibHandle) -> Option<LayerCommand> {
        let resources = EcsResources {
            time: &ctx.time,
            actions: &ctx.actions,
        };

        self.ecs.run_system(&resources, movement);

        if ctx.actions.take(Action::Confirm) {
            return Some(LayerCommand::Replace(Box::new(MenuLayer)));
        }

        if ctx.actions.take(Action::Pause) {
            return Some(LayerCommand::Push(Box::new(PauseLayer)));
        }

        if ctx.actions.take(Action::Quit) {
            return Some(LayerCommand::Quit);
        }

        None
    }

    fn on_render(&mut self, ctx: &AppContext, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);

        self.ecs.run_render_system(d, draw_positions);
    }

    fn on_attach(&mut self, ctx: &mut AppContext) {
        println!("Attaching game layer...");
    }

    fn on_detach(&mut self, ctx: &mut AppContext) {
        println!("Detaching game layer...");
    }
}
