use crate::app::layers::menu::MenuLayer;
use crate::app::layers::pause::PauseLayer;
use crate::core::action::*;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct GameLayer;

impl Layer for GameLayer {
    fn on_event(&mut self, ctx: &mut AppContext, event: &Event) {}

    fn on_update(&mut self, ctx: &mut AppContext, rl: &mut RaylibHandle) -> Option<LayerCommand> {
        if ctx.actions.contains(Action::Confirm) {
            return Some(LayerCommand::Replace(Box::new(MenuLayer)));
        }

        if ctx.actions.contains(Action::Pause) {
            return Some(LayerCommand::Push(Box::new(PauseLayer)));
        }

        if ctx.actions.contains(Action::Quit) {
            return Some(LayerCommand::Quit);
        }

        None
    }

    fn on_render(&mut self, ctx: &AppContext, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);
    }

    fn on_attach(&mut self, ctx: &mut AppContext) {
        println!("Attaching game layer...");
    }

    fn on_detach(&mut self, ctx: &mut AppContext) {
        println!("Detaching game layer...");
    }
}
