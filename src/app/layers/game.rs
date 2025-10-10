use crate::app::layers::menu::MenuLayer;
use crate::core::application::*;
use raylib::prelude::*;

pub struct GameLayer;

impl Layer for GameLayer {
    fn update(&mut self, rl: &mut RaylibHandle) -> LayerControl {
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            return LayerControl::quit();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return LayerControl::change_layer(Some(Box::new(MenuLayer)));
        }

        LayerControl::continue_running()
    }

    fn render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);
    }
}
