use crate::app::action::*;
use crate::app::layers::game::GameLayer;
use crate::engine::context::*;
use crate::engine::event::*;
use crate::engine::layer::*;
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
        d.draw_text("Main Menu", 20, 20, 20, Color::BLUE);
        d.draw_text("Space -> Game", 20, 50, 20, Color::BLACK);
        d.draw_text("Q -> Quit", 20, 80, 20, Color::BLACK);
    }

    fn on_attach(&mut self, ctx: &mut AppContext<Action>) {}

    fn on_detach(&mut self, ctx: &mut AppContext<Action>) {}
}
