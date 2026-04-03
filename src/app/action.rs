use crate::core::action::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Confirm,
    Pause,
    Quit,
    Up,
    Down,
    Left,
    Right,
}

impl ActionType for Action {
    fn default_key_bindings() -> Vec<(KeyboardKey, Self)> {
        vec![
            (KeyboardKey::KEY_SPACE, Action::Confirm),
            (KeyboardKey::KEY_ENTER, Action::Confirm),
            (KeyboardKey::KEY_P, Action::Pause),
            (KeyboardKey::KEY_Q, Action::Quit),
            // Movement - WASD
            (KeyboardKey::KEY_W, Action::Up),
            (KeyboardKey::KEY_S, Action::Down),
            (KeyboardKey::KEY_A, Action::Left),
            (KeyboardKey::KEY_D, Action::Right),
            // Movement - Arrow keys
            (KeyboardKey::KEY_UP, Action::Up),
            (KeyboardKey::KEY_DOWN, Action::Down),
            (KeyboardKey::KEY_LEFT, Action::Left),
            (KeyboardKey::KEY_RIGHT, Action::Right),
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
            // Movement - D-pad
            (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP, Action::Up),
            (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN, Action::Down),
            (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT, Action::Left),
            (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT, Action::Right),
        ]
    }

    fn is_continuous(&self) -> bool {
        matches!(
            self,
            Action::Up | Action::Down | Action::Left | Action::Right
        )
    }
}
