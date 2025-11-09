use crate::app::layers::menu::MenuLayer;
use crate::app::layers::pause::PauseLayer;
use crate::core::application::*;
use raylib::prelude::*;

pub struct GameLayer;

impl Layer for GameLayer {
    fn on_update(&mut self, rl: &mut RaylibHandle) -> LayerCommand {
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            return LayerCommand::Quit;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return LayerCommand::Replace(Box::new(MenuLayer));
        }

        if rl.is_key_pressed(KeyboardKey::KEY_M) {
            return LayerCommand::Push(Box::new(PauseLayer));
        }

        LayerCommand::None
    }

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);
    }
}
