use crate::core::action::*;
use raylib::prelude::*;
use std::collections::HashMap;

pub struct Settings {
    pub input_bindings: InputBindings,
}

impl Settings {
    pub fn default() -> Self {
        let default_bindings = InputBindings::default();

        Self {
            input_bindings: default_bindings,
        }
    }
}

pub struct InputBindings {
    key_bindings: HashMap<KeyboardKey, Action>,
}

impl InputBindings {
    pub fn default() -> Self {
        let mut default_keys = HashMap::new();

        default_keys.insert(KeyboardKey::KEY_ENTER, Action::Confirm);
        default_keys.insert(KeyboardKey::KEY_SPACE, Action::Confirm);
        default_keys.insert(KeyboardKey::KEY_P, Action::Pause);
        default_keys.insert(KeyboardKey::KEY_Q, Action::Quit);

        Self {
            key_bindings: default_keys,
        }
    }

    pub fn key_bindings(&self) -> &HashMap<KeyboardKey, Action> {
        &self.key_bindings
    }
}
