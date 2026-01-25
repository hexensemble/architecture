use crate::app::action::*;
use crate::app::layers::menu::*;
use crate::core::layer::*;

pub mod action;
pub mod ecs;
pub mod layers;

pub fn initial_layer() -> Box<dyn Layer<Action>> {
    Box::new(MenuLayer)
}
