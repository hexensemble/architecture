use crate::core::application::Layer;
use crate::core::state::StateControl;
use raylib::prelude::*;

pub struct GameUiLayer;

impl Layer for GameUiLayer {
    fn update(&self, rl: &mut RaylibHandle) -> StateControl {
        StateControl::Continue
    }

    fn render(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
}
