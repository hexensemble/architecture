use crate::core::action::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Confirm,
    Pause,
    Quit,
}

impl ActionType for Action {
    fn default_key_bindings() -> Vec<(KeyboardKey, Self)> {
        vec![
            (KeyboardKey::KEY_SPACE, Action::Confirm),
            (KeyboardKey::KEY_ENTER, Action::Confirm),
            (KeyboardKey::KEY_P, Action::Pause),
            (KeyboardKey::KEY_Q, Action::Quit),
        ]
    }

    fn default_pad_bindings() -> Vec<(GamepadButton, Self)> {
        Vec::new()
    }
}
