use crate::app::layers::menu::*;
use crate::core::layer::*;

pub mod ecs;
pub mod keys;
pub mod layers;

pub fn initial_layer() -> Box<dyn Layer> {
    Box::new(MenuLayer)
}
