use crate::engine::action::*;
use crate::engine::bindings::*;
use crate::engine::settings::*;
use crate::engine::time::*;

pub struct AppContext<A: ActionType> {
    pub settings: Settings<A>,
    pub bindings: InputBindings<A>,
    pub actions: Actions<A>,
    pub time: Time,
}
