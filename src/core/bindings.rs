use crate::core::action::*;
use raylib::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

pub struct InputBindings<A: ActionType> {
    key_bindings: HashMap<KeyboardKey, A>,
    pad_bindings: HashMap<GamepadButton, A>,
}

impl<A: ActionType> InputBindings<A> {
    pub fn new(serialized_bindings: &SerializedBindings<A>) -> Self {
        let mut keys: HashMap<KeyboardKey, A> = HashMap::new();
        for (serialized_key, action) in &serialized_bindings.key_bindings {
            keys.insert(serialized_key.0, *action);
        }

        let mut pads: HashMap<GamepadButton, A> = HashMap::new();
        for (serialized_pad, action) in &serialized_bindings.pad_bindings {
            pads.insert(serialized_pad.0, *action);
        }

        Self {
            key_bindings: keys,
            pad_bindings: pads,
        }
    }

    pub fn key_bindings(&self) -> &HashMap<KeyboardKey, A> {
        &self.key_bindings
    }

    pub fn pad_bindings(&self) -> &HashMap<GamepadButton, A> {
        &self.pad_bindings
    }
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "A: DeserializeOwned"))]
pub struct SerializedBindings<A: ActionType> {
    pub key_bindings: HashMap<SerializeableKey, A>,
    pub pad_bindings: HashMap<SerializeablePad, A>,
}

impl<A: ActionType> Default for SerializedBindings<A> {
    fn default() -> Self {
        let mut keys = HashMap::new();
        for (key, action) in A::default_key_bindings() {
            keys.insert(SerializeableKey(key), action);
        }

        let mut pads = HashMap::new();
        for (pad, action) in A::default_pad_bindings() {
            pads.insert(SerializeablePad(pad), action);
        }

        Self {
            key_bindings: keys,
            pad_bindings: pads,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerializeableKey(pub KeyboardKey);

impl Display for SerializeableKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl FromStr for SerializeableKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s {
            // Letters
            "KEY_A" => KeyboardKey::KEY_A,
            "KEY_B" => KeyboardKey::KEY_B,
            "KEY_C" => KeyboardKey::KEY_C,
            "KEY_D" => KeyboardKey::KEY_D,
            "KEY_E" => KeyboardKey::KEY_E,
            "KEY_F" => KeyboardKey::KEY_F,
            "KEY_G" => KeyboardKey::KEY_G,
            "KEY_H" => KeyboardKey::KEY_H,
            "KEY_I" => KeyboardKey::KEY_I,
            "KEY_J" => KeyboardKey::KEY_J,
            "KEY_K" => KeyboardKey::KEY_K,
            "KEY_L" => KeyboardKey::KEY_L,
            "KEY_M" => KeyboardKey::KEY_M,
            "KEY_N" => KeyboardKey::KEY_N,
            "KEY_O" => KeyboardKey::KEY_O,
            "KEY_P" => KeyboardKey::KEY_P,
            "KEY_Q" => KeyboardKey::KEY_Q,
            "KEY_R" => KeyboardKey::KEY_R,
            "KEY_S" => KeyboardKey::KEY_S,
            "KEY_T" => KeyboardKey::KEY_T,
            "KEY_U" => KeyboardKey::KEY_U,
            "KEY_V" => KeyboardKey::KEY_V,
            "KEY_W" => KeyboardKey::KEY_W,
            "KEY_X" => KeyboardKey::KEY_X,
            "KEY_Y" => KeyboardKey::KEY_Y,
            "KEY_Z" => KeyboardKey::KEY_Z,

            // Numbers
            "KEY_ZERO" => KeyboardKey::KEY_ZERO,
            "KEY_ONE" => KeyboardKey::KEY_ONE,
            "KEY_TWO" => KeyboardKey::KEY_TWO,
            "KEY_THREE" => KeyboardKey::KEY_THREE,
            "KEY_FOUR" => KeyboardKey::KEY_FOUR,
            "KEY_FIVE" => KeyboardKey::KEY_FIVE,
            "KEY_SIX" => KeyboardKey::KEY_SIX,
            "KEY_SEVEN" => KeyboardKey::KEY_SEVEN,
            "KEY_EIGHT" => KeyboardKey::KEY_EIGHT,
            "KEY_NINE" => KeyboardKey::KEY_NINE,

            // Function keys
            "KEY_F1" => KeyboardKey::KEY_F1,
            "KEY_F2" => KeyboardKey::KEY_F2,
            "KEY_F3" => KeyboardKey::KEY_F3,
            "KEY_F4" => KeyboardKey::KEY_F4,
            "KEY_F5" => KeyboardKey::KEY_F5,
            "KEY_F6" => KeyboardKey::KEY_F6,
            "KEY_F7" => KeyboardKey::KEY_F7,
            "KEY_F8" => KeyboardKey::KEY_F8,
            "KEY_F9" => KeyboardKey::KEY_F9,
            "KEY_F10" => KeyboardKey::KEY_F10,
            "KEY_F11" => KeyboardKey::KEY_F11,
            "KEY_F12" => KeyboardKey::KEY_F12,

            // Special keys
            "KEY_SPACE" => KeyboardKey::KEY_SPACE,
            "KEY_ENTER" => KeyboardKey::KEY_ENTER,
            "KEY_ESCAPE" => KeyboardKey::KEY_ESCAPE,
            "KEY_TAB" => KeyboardKey::KEY_TAB,
            "KEY_BACKSPACE" => KeyboardKey::KEY_BACKSPACE,
            "KEY_INSERT" => KeyboardKey::KEY_INSERT,
            "KEY_DELETE" => KeyboardKey::KEY_DELETE,

            // Arrow keys
            "KEY_UP" => KeyboardKey::KEY_UP,
            "KEY_DOWN" => KeyboardKey::KEY_DOWN,
            "KEY_LEFT" => KeyboardKey::KEY_LEFT,
            "KEY_RIGHT" => KeyboardKey::KEY_RIGHT,

            // Navigation
            "KEY_HOME" => KeyboardKey::KEY_HOME,
            "KEY_END" => KeyboardKey::KEY_END,
            "KEY_PAGE_UP" => KeyboardKey::KEY_PAGE_UP,
            "KEY_PAGE_DOWN" => KeyboardKey::KEY_PAGE_DOWN,

            // Modifiers
            "KEY_LEFT_SHIFT" => KeyboardKey::KEY_LEFT_SHIFT,
            "KEY_RIGHT_SHIFT" => KeyboardKey::KEY_RIGHT_SHIFT,
            "KEY_LEFT_CONTROL" => KeyboardKey::KEY_LEFT_CONTROL,
            "KEY_RIGHT_CONTROL" => KeyboardKey::KEY_RIGHT_CONTROL,
            "KEY_LEFT_ALT" => KeyboardKey::KEY_LEFT_ALT,
            "KEY_RIGHT_ALT" => KeyboardKey::KEY_RIGHT_ALT,
            "KEY_LEFT_SUPER" => KeyboardKey::KEY_LEFT_SUPER,
            "KEY_RIGHT_SUPER" => KeyboardKey::KEY_RIGHT_SUPER,

            // Lock keys
            "KEY_CAPS_LOCK" => KeyboardKey::KEY_CAPS_LOCK,
            "KEY_SCROLL_LOCK" => KeyboardKey::KEY_SCROLL_LOCK,
            "KEY_NUM_LOCK" => KeyboardKey::KEY_NUM_LOCK,
            "KEY_PRINT_SCREEN" => KeyboardKey::KEY_PRINT_SCREEN,
            "KEY_PAUSE" => KeyboardKey::KEY_PAUSE,

            // Punctuation/symbols
            "KEY_APOSTROPHE" => KeyboardKey::KEY_APOSTROPHE,
            "KEY_COMMA" => KeyboardKey::KEY_COMMA,
            "KEY_MINUS" => KeyboardKey::KEY_MINUS,
            "KEY_PERIOD" => KeyboardKey::KEY_PERIOD,
            "KEY_SLASH" => KeyboardKey::KEY_SLASH,
            "KEY_SEMICOLON" => KeyboardKey::KEY_SEMICOLON,
            "KEY_EQUAL" => KeyboardKey::KEY_EQUAL,
            "KEY_LEFT_BRACKET" => KeyboardKey::KEY_LEFT_BRACKET,
            "KEY_RIGHT_BRACKET" => KeyboardKey::KEY_RIGHT_BRACKET,
            "KEY_BACKSLASH" => KeyboardKey::KEY_BACKSLASH,
            "KEY_GRAVE" => KeyboardKey::KEY_GRAVE,

            // Keypad
            "KEY_KP_0" => KeyboardKey::KEY_KP_0,
            "KEY_KP_1" => KeyboardKey::KEY_KP_1,
            "KEY_KP_2" => KeyboardKey::KEY_KP_2,
            "KEY_KP_3" => KeyboardKey::KEY_KP_3,
            "KEY_KP_4" => KeyboardKey::KEY_KP_4,
            "KEY_KP_5" => KeyboardKey::KEY_KP_5,
            "KEY_KP_6" => KeyboardKey::KEY_KP_6,
            "KEY_KP_7" => KeyboardKey::KEY_KP_7,
            "KEY_KP_8" => KeyboardKey::KEY_KP_8,
            "KEY_KP_9" => KeyboardKey::KEY_KP_9,
            "KEY_KP_DECIMAL" => KeyboardKey::KEY_KP_DECIMAL,
            "KEY_KP_DIVIDE" => KeyboardKey::KEY_KP_DIVIDE,
            "KEY_KP_MULTIPLY" => KeyboardKey::KEY_KP_MULTIPLY,
            "KEY_KP_SUBTRACT" => KeyboardKey::KEY_KP_SUBTRACT,
            "KEY_KP_ADD" => KeyboardKey::KEY_KP_ADD,
            "KEY_KP_ENTER" => KeyboardKey::KEY_KP_ENTER,
            "KEY_KP_EQUAL" => KeyboardKey::KEY_KP_EQUAL,

            // Menu
            "KEY_KB_MENU" => KeyboardKey::KEY_KB_MENU,

            // Unknown
            _ => return Err(format!("Unknown key: {}", s)),
        };

        Ok(SerializeableKey(key))
    }
}

impl Serialize for SerializeableKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SerializeableKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SerializeableKey::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerializeablePad(pub GamepadButton);

impl Display for SerializeablePad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl FromStr for SerializeablePad {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pad = match s {
            // Face buttons
            "GAMEPAD_BUTTON_RIGHT_FACE_UP" => GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP, // Y/Triangle
            "GAMEPAD_BUTTON_RIGHT_FACE_RIGHT" => GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT, // B/Circle
            "GAMEPAD_BUTTON_RIGHT_FACE_DOWN" => GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN, // A/Cross
            "GAMEPAD_BUTTON_RIGHT_FACE_LEFT" => GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT, // X/Square

            // D-pad
            "GAMEPAD_BUTTON_LEFT_FACE_UP" => GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP,
            "GAMEPAD_BUTTON_LEFT_FACE_RIGHT" => GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
            "GAMEPAD_BUTTON_LEFT_FACE_DOWN" => GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN,
            "GAMEPAD_BUTTON_LEFT_FACE_LEFT" => GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT,

            // Triggers/bumpers
            "GAMEPAD_BUTTON_LEFT_TRIGGER_1" => GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1, // LB
            "GAMEPAD_BUTTON_LEFT_TRIGGER_2" => GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2, // LT
            "GAMEPAD_BUTTON_RIGHT_TRIGGER_1" => GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1, // RB
            "GAMEPAD_BUTTON_RIGHT_TRIGGER_2" => GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2, // RT

            // Thumbstick buttons
            "GAMEPAD_BUTTON_LEFT_THUMB" => GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB, // L3
            "GAMEPAD_BUTTON_RIGHT_THUMB" => GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB, // R3

            // Center buttons
            "GAMEPAD_BUTTON_MIDDLE_LEFT" => GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT, // Select/Back
            "GAMEPAD_BUTTON_MIDDLE" => GamepadButton::GAMEPAD_BUTTON_MIDDLE,           // Guide/Home
            "GAMEPAD_BUTTON_MIDDLE_RIGHT" => GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT, // Start

            // Unknown
            _ => return Err(format!("Unknown key: {}", s)),
        };

        Ok(SerializeablePad(pad))
    }
}

impl Serialize for SerializeablePad {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SerializeablePad {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SerializeablePad::from_str(&s).map_err(serde::de::Error::custom)
    }
}
