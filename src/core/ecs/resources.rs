use crate::core::action::*;
use crate::core::time::*;

pub struct EcsResources<'a, A: ActionType> {
    pub time: &'a Time,
    pub actions: &'a Actions<A>,
}
