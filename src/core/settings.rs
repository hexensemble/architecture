use crate::core::action::*;
use raylib::prelude::*;
use std::collections::HashMap;

pub struct Settings<A: ActionType> {
    pub bindings: InputBindings<A>,
}

impl<A: ActionType> Settings<A> {
    pub fn default() -> Self {
        let default_bindings = InputBindings::default();

        Self {
            bindings: default_bindings,
        }
    }
}

pub struct InputBindings<A: ActionType> {
    key_bindings: HashMap<KeyboardKey, A>,
    pad_bindings: HashMap<GamepadButton, A>,
}

impl<A: ActionType> InputBindings<A> {
    pub fn default() -> Self {
        Self {
            key_bindings: HashMap::new(),
            pad_bindings: HashMap::new(),
        }
    }

    pub fn new(
        key_bindings: HashMap<KeyboardKey, A>,
        pad_bindings: HashMap<GamepadButton, A>,
    ) -> Self {
        Self {
            key_bindings,
            pad_bindings,
        }
    }

    pub fn key_bindings(&self) -> &HashMap<KeyboardKey, A> {
        &self.key_bindings
    }

    pub fn pad_bindings(&self) -> &HashMap<GamepadButton, A> {
        &self.pad_bindings
    }
}
