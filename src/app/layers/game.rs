use crate::app::layers::menu::MenuLayer;
use crate::app::layers::pause::PauseLayer;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct GameLayer;

impl Layer for GameLayer {
    fn on_event(&mut self, event: &Event) -> Option<LayerCommand> {
        match event {
            Event::KeyPressed(KeyboardKey::KEY_Q) => Some(LayerCommand::Quit),
            Event::KeyPressed(KeyboardKey::KEY_SPACE) => {
                Some(LayerCommand::Replace(Box::new(MenuLayer)))
            }
            Event::KeyPressed(KeyboardKey::KEY_M) => Some(LayerCommand::Push(Box::new(PauseLayer))),
            _ => None,
        }
    }

    fn on_update(&mut self, rl: &mut RaylibHandle) {}

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);
    }

    fn on_attach(&mut self) {
        println!("Attaching game layer...");
    }

    fn on_detach(&mut self) {
        println!("Detaching game layer...");
    }
}
