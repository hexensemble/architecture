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
    pub net_settings: NetSettings,
}

impl<A: ActionType> Default for Settings<A> {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
            fps: 30,
            serialized_bindings: SerializedBindings::default(),
            net_settings: NetSettings::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetSettings {
    pub mode: NetMode,
    pub server_addr: String,
}

impl Default for NetSettings {
    fn default() -> Self {
        Self {
            mode: NetMode::Local,
            server_addr: "127.0.0.1:27960".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NetMode {
    Local,
    Remote,
}
