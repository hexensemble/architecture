use crate::engine::action::*;
use crate::engine::bindings::*;
use raylib::prelude::*;

pub enum Event {
    KeyPressed(KeyboardKey),
    KeyDown(KeyboardKey),
    PadPressed(i32, GamepadButton),
    PadDown(i32, GamepadButton),
    MousePosition(Vector2),
}

pub fn collect_events<A: ActionType>(rl: &RaylibHandle, bindings: &InputBindings<A>) -> Vec<Event> {
    let mut events = Vec::new();

    for (key, action) in bindings.key_bindings().iter() {
        if action.is_continuous() {
            if rl.is_key_down(*key) {
                events.push(Event::KeyDown(*key));
            }
        } else {
            if rl.is_key_pressed(*key) {
                events.push(Event::KeyPressed(*key));
            }
        }
    }

    for gamepad_id in 0..4 {
        if rl.is_gamepad_available(gamepad_id) {
            for (pad, action) in bindings.pad_bindings().iter() {
                if action.is_continuous() {
                    if rl.is_gamepad_button_down(gamepad_id, *pad) {
                        events.push(Event::PadDown(gamepad_id, *pad));
                    }
                } else {
                    if rl.is_gamepad_button_pressed(gamepad_id, *pad) {
                        events.push(Event::PadPressed(gamepad_id, *pad));
                    }
                }
            }
        }
    }

    let pos = rl.get_mouse_position();
    events.push(Event::MousePosition(pos));

    events
}
