use crate::core::action::*;
use crate::core::settings::*;
use crate::core::time::*;

pub struct AppContext<A: ActionType> {
    pub settings: Settings<A>,
    pub actions: Actions<A>,
    pub time: Time,
}
