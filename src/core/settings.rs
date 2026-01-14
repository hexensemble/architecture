use crate::core::action::*;
use raylib::prelude::*;
use std::collections::HashMap;

pub struct Settings {
    pub bindings: InputBindings,
}

impl Settings {
    pub fn default() -> Self {
        let default_bindings = InputBindings::default();

        Self {
            bindings: default_bindings,
        }
    }
}

pub struct InputBindings {
    key_bindings: HashMap<KeyboardKey, Action>,
    pad_bindings: HashMap<GamepadButton, Action>,
}

impl InputBindings {
    pub fn default() -> Self {
        Self {
            key_bindings: HashMap::new(),
            pad_bindings: HashMap::new(),
        }
    }

    pub fn new(
        key_bindings: HashMap<KeyboardKey, Action>,
        pad_bindings: HashMap<GamepadButton, Action>,
    ) -> Self {
        Self {
            key_bindings,
            pad_bindings,
        }
    }

    pub fn key_bindings(&self) -> &HashMap<KeyboardKey, Action> {
        &self.key_bindings
    }

    pub fn pad_bindings(&self) -> &HashMap<GamepadButton, Action> {
        &self.pad_bindings
    }
}
