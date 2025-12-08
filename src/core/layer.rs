use crate::core::event::*;
use raylib::prelude::*;

pub trait Layer {
    fn on_event(&mut self, event: &Event) -> Option<LayerCommand>;
    fn on_update(&mut self, rl: &mut RaylibHandle);
    fn on_render(&mut self, d: &mut RaylibDrawHandle);
}

pub enum LayerCommand {
    None,
    Push(Box<dyn Layer>),
    Pop,
    Replace(Box<dyn Layer>),
    Quit,
}
