use crate::core::action::*;
use crate::core::time::*;

pub struct EcsResources<'a> {
    pub time: &'a Time,
    pub actions: &'a Actions,
}
