use crate::app::layers::game::GameLayer;
use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct MenuLayer;

impl Layer for MenuLayer {
    fn on_event(&mut self, event: &Event) -> Option<LayerCommand> {
        match event {
            Event::KeyPressed(KeyboardKey::KEY_Q) => Some(LayerCommand::Quit),
            Event::KeyPressed(KeyboardKey::KEY_SPACE) => {
                Some(LayerCommand::Replace(Box::new(GameLayer)))
            }
            _ => None,
        }
    }

    fn on_update(&mut self, rl: &mut RaylibHandle) {}

    fn on_render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the menu layer!", 12, 12, 20, Color::BLACK);
    }

    fn on_attach(&mut self) {
        println!("Attaching menu layer...");
    }

    fn on_detach(&mut self) {
        println!("Detaching menu layer...");
    }
}
