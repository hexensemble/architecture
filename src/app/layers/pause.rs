use crate::core::action::*;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct PauseLayer;

impl Layer for PauseLayer {
    fn on_event(&mut self, ctx: &mut AppContext, event: &Event) {}

    fn on_update(&mut self, ctx: &mut AppContext, rl: &mut RaylibHandle) -> Option<LayerCommand> {
        if ctx.actions.take(Action::Pause) {
            return Some(LayerCommand::Pop);
        } else {
            return Some(LayerCommand::None);
        }
    }

    fn on_render(&mut self, ctx: &AppContext, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the pause layer!", 200, 200, 30, Color::RED);
    }

    fn on_attach(&mut self, ctx: &mut AppContext) {
        println!("Attaching pause layer...");
    }

    fn on_detach(&mut self, ctx: &mut AppContext) {
        println!("Detaching pause layer...");
    }
}
