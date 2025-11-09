use crate::core::application::*;
use raylib::prelude::*;

pub struct PauseLayer;

impl Layer for PauseLayer {
    fn on_update(&mut self, rl: &mut RaylibHandle) -> LayerCommand {
        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            return LayerCommand::Pop;
        }

        LayerCommand::None
    }

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 200, 200, 30, Color::RED);
    }
}
