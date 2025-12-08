use crate::app::keys::*;
use raylib::prelude::*;

pub enum Event {
    KeyPressed(KeyboardKey),
    MousePosition(Vector2),
}

pub fn collect_events(rl: &RaylibHandle) -> Vec<Event> {
    let mut events = Vec::new();

    for key in KEYS {
        if rl.is_key_pressed(*key) {
            events.push(Event::KeyPressed(*key));
        }
    }

    let pos = rl.get_mouse_position();
    events.push(Event::MousePosition(pos));

    events
}
