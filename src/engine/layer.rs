use crate::engine::action::*;
use crate::engine::context::*;
use crate::engine::event::*;
use raylib::prelude::*;

pub trait Layer<A: ActionType> {
    fn on_event(&mut self, ctx: &mut AppContext<A>, event: &Event);
    fn on_update(
        &mut self,
        ctx: &mut AppContext<A>,
        rl: &mut RaylibHandle,
    ) -> Option<LayerCommand<A>>;
    fn on_render(&mut self, ctx: &AppContext<A>, d: &mut RaylibDrawHandle);

    fn on_attach(&mut self, ctx: &mut AppContext<A>);
    fn on_detach(&mut self, ctx: &mut AppContext<A>);
}

pub enum LayerCommand<A: ActionType> {
    None,
    Push(Box<dyn Layer<A>>),
    Pop,
    Replace(Box<dyn Layer<A>>),
    Quit,
}
