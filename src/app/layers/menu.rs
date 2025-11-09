use crate::app::layers::game::GameLayer;
use crate::core::application::*;
use raylib::prelude::*;

pub struct MenuLayer;

impl Layer for MenuLayer {
    fn on_update(&mut self, rl: &mut RaylibHandle) -> LayerCommand {
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            return LayerCommand::Quit;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return LayerCommand::Replace(Box::new(GameLayer));
        }

        LayerCommand::None
    }

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 12, 12, 20, Color::BLACK);
    }
}
