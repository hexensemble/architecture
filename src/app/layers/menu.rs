use crate::app::action::*;
use crate::app::layers::game::GameLayer;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use crate::net::session::*;
use raylib::prelude::*;

pub struct MenuLayer;

impl Layer<Action> for MenuLayer {
    fn on_event(&mut self, ctx: &mut AppContext<Action>, event: &Event) {}

    fn on_update(
        &mut self,
        ctx: &mut AppContext<Action>,
        rl: &mut RaylibHandle,
    ) -> Option<LayerCommand<Action>> {
        if ctx.actions.take(Action::Confirm) {
            match make_session(&ctx.settings.net_settings) {
                Ok(session) => {
                    return Some(LayerCommand::Replace(Box::new(GameLayer::new(session))));
                }
                Err(e) => {
                    log::error!("Failed to create game session: {}", e);
                    return None;
                }
            }
        }

        if ctx.actions.take(Action::Quit) {
            return Some(LayerCommand::Quit);
        }

        None
    }

    fn on_render(&mut self, ctx: &AppContext<Action>, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 12, 12, 20, Color::BLACK);
    }

    fn on_attach(&mut self, ctx: &mut AppContext<Action>) {
        println!("Attaching menu layer...");
    }

    fn on_detach(&mut self, ctx: &mut AppContext<Action>) {
        println!("Detaching menu layer...");
    }
}
