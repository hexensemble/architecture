use crate::app::layers::game::GameLayer;
use crate::core::action::*;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct MenuLayer;

impl Layer for MenuLayer {
    fn on_event(&mut self, ctx: &mut AppContext, event: &Event) {}

    fn on_update(&mut self, ctx: &mut AppContext, rl: &mut RaylibHandle) -> Option<LayerCommand> {
        if ctx.actions.take(Action::Confirm) {
            return Some(LayerCommand::Replace(Box::new(GameLayer)));
        }

        if ctx.actions.take(Action::Quit) {
            return Some(LayerCommand::Quit);
        }

        None
    }

    fn on_render(&mut self, ctx: &AppContext, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 12, 12, 20, Color::BLACK);
    }

    fn on_attach(&mut self, ctx: &mut AppContext) {
        println!("Attaching menu layer...");
    }

    fn on_detach(&mut self, ctx: &mut AppContext) {
        println!("Detaching menu layer...");
    }
}
