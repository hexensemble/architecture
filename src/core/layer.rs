use crate::core::context::*;
use crate::core::event::*;
use raylib::prelude::*;

pub trait Layer {
    fn on_event(&mut self, ctx: &mut AppContext, event: &Event);
    fn on_update(&mut self, ctx: &mut AppContext, rl: &mut RaylibHandle) -> Option<LayerCommand>;
    fn on_render(&mut self, ctx: &AppContext, d: &mut RaylibDrawHandle);

    fn on_attach(&mut self, ctx: &mut AppContext);
    fn on_detach(&mut self, ctx: &mut AppContext);
}

pub enum LayerCommand {
    None,
    Push(Box<dyn Layer>),
    Pop,
    Replace(Box<dyn Layer>),
    Quit,
}
