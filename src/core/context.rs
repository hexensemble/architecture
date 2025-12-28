use crate::core::action::*;
use crate::core::settings::*;
use crate::core::time::*;

pub struct AppContext {
    pub settings: Settings,
    pub actions: Actions,
    pub time: Time,
}
