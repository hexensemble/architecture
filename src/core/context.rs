use crate::core::action::*;
use crate::core::bindings::*;
use crate::core::settings::*;
use crate::core::time::*;

pub struct AppContext<A: ActionType> {
    pub settings: Settings<A>,
    pub bindings: InputBindings<A>,
    pub actions: Actions<A>,
    pub time: Time,
}
