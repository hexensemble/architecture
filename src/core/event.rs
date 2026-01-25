use crate::core::action::*;
use crate::core::bindings::*;
use raylib::prelude::*;

pub enum Event {
    KeyPressed(KeyboardKey),
    PadPressed(GamepadButton),
    MousePosition(Vector2),
}

pub fn collect_events<A: ActionType>(rl: &RaylibHandle, bindings: &InputBindings<A>) -> Vec<Event> {
    let mut events = Vec::new();

    for key in bindings.key_bindings().keys() {
        if rl.is_key_pressed(*key) {
            events.push(Event::KeyPressed(*key));
        }
    }

    for pad in bindings.pad_bindings().keys() {
        if rl.is_gamepad_button_pressed(0, *pad) {
            events.push(Event::PadPressed(*pad));
        }
    }

    let pos = rl.get_mouse_position();
    events.push(Event::MousePosition(pos));

    events
}
