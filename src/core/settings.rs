use crate::core::action::*;
use crate::core::bindings::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "A: DeserializeOwned"))]
pub struct Settings<A: ActionType> {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
    pub serialized_bindings: SerializedBindings<A>,
}

impl<A: ActionType> Default for Settings<A> {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
            fps: 30,
            serialized_bindings: SerializedBindings::default(),
        }
    }
}
