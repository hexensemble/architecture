use crate::core::application::Application;
use crate::core::state::State;

pub struct StateManager {
    pub current: Option<State>,
    pub next: Option<State>,
    pub change: bool,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            current: None,
            next: None,
            change: false,
        }
    }

    pub fn set_state(&mut self, state: State, app: &mut Application) {
        self.current = Some(state);

        app.clear_layers();

        if let Some(state) = &self.current {
            for layer in state.layers() {
                app.push_layer(layer);
            }
        }
    }
}
