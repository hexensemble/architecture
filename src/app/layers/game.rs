use crate::core::application::Layer;
use crate::core::state::{State, StateControl};
use raylib::prelude::*;

pub struct GameLayer;

impl Layer for GameLayer {
    fn update(&self, rl: &mut RaylibHandle) -> StateControl {
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            return StateControl::Stop;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return StateControl::Change(State::Menu);
        }

        StateControl::Continue
    }

    fn render(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RED);
    }
}
