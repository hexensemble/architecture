use crate::app::action::*;
use crate::engine::context::*;
use crate::engine::event::*;
use crate::engine::layer::*;
use raylib::prelude::*;

pub struct PauseLayer;

impl Layer<Action> for PauseLayer {
    fn on_event(&mut self, ctx: &mut AppContext<Action>, event: &Event) {}

    fn on_update(
        &mut self,
        ctx: &mut AppContext<Action>,
        rl: &mut RaylibHandle,
    ) -> Option<LayerCommand<Action>> {
        if ctx.actions.take(Action::Pause) {
            Some(LayerCommand::Pop)
        } else {
            Some(LayerCommand::None)
        }
    }

    fn on_render(&mut self, ctx: &AppContext<Action>, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Paused",
            (ctx.settings.width / 2) - 55,
            (ctx.settings.height / 2) - 15,
            30,
            Color::RED,
        );
    }

    fn on_attach(&mut self, ctx: &mut AppContext<Action>) {}

    fn on_detach(&mut self, ctx: &mut AppContext<Action>) {}
}
