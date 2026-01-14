use crate::core::action::*;
use raylib::prelude::*;
use std::collections::HashMap;

pub fn key_bindings() -> HashMap<KeyboardKey, Action> {
    let mut keys = HashMap::new();

    keys.insert(KeyboardKey::KEY_ENTER, Action::Confirm);
    keys.insert(KeyboardKey::KEY_SPACE, Action::Confirm);
    keys.insert(KeyboardKey::KEY_P, Action::Pause);
    keys.insert(KeyboardKey::KEY_Q, Action::Quit);

    keys
}

//TODO
pub fn pad_bindings() -> HashMap<GamepadButton, Action> {
    let mut pads = HashMap::new();

    pads
}
