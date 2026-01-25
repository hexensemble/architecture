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
        vec![
            (
                GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
                Action::Confirm,
            ),
            (
                GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
                Action::Pause,
            ),
            (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT, Action::Quit),
        ]
    }
}
