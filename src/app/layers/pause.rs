use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct PauseLayer;

impl Layer for PauseLayer {
    fn on_event(&mut self, event: &Event) -> Option<LayerCommand> {
        match event {
            Event::KeyPressed(KeyboardKey::KEY_M) => Some(LayerCommand::Pop),
            _ => Some(LayerCommand::None),
        }
    }

    fn on_update(&mut self, rl: &mut RaylibHandle) {}

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 200, 200, 30, Color::RED);
    }
}
