use crate::core::application::Layer;
use crate::core::state::StateControl;
use raylib::prelude::*;

pub struct MenuUiLayer;

impl Layer for MenuUiLayer {
    fn update(&self, rl: &mut RaylibHandle) -> StateControl {
        StateControl::Continue
    }

    fn render(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
}
