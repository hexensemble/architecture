use crate::core::settings::*;
use raylib::prelude::*;

pub enum Event {
    KeyPressed(KeyboardKey),
    MousePosition(Vector2),
}

pub fn collect_events(rl: &RaylibHandle, bindings: &InputBindings) -> Vec<Event> {
    let mut events = Vec::new();

    for key in bindings.key_bindings().keys() {
        if rl.is_key_pressed(*key) {
            events.push(Event::KeyPressed(*key));
        }
    }

    let pos = rl.get_mouse_position();
    events.push(Event::MousePosition(pos));

    events
}
