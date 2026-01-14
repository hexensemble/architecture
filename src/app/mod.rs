use crate::app::layers::menu::*;
use crate::core::layer::*;
use crate::core::settings::*;

pub mod bindings;
pub mod ecs;
pub mod layers;

pub fn initial_layer() -> Box<dyn Layer> {
    Box::new(MenuLayer)
}

pub fn bindings() -> InputBindings {
    InputBindings::new(bindings::key_bindings(), bindings::pad_bindings())
}
