use crate::app::layers::{
    game::GameLayer, game_ui::GameUiLayer, menu::MenuLayer, menu_ui::MenuUiLayer,
};
use crate::core::application::Layer;

#[derive(Copy, Clone)]
pub enum State {
    Menu,
    Game,
}

impl State {
    pub fn layers(&self) -> Vec<Box<dyn Layer>> {
        match self {
            State::Menu => vec![Box::new(MenuLayer), Box::new(MenuUiLayer)],
            State::Game => vec![Box::new(GameLayer), Box::new(GameUiLayer)],
        }
    }
}

pub enum StateControl {
    Continue,
    Stop,
    Change(State),
}
