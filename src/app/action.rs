use crate::core::action::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Confirm,
    Pause,
    Quit,
}

impl ActionType for Action {}
