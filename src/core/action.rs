#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Confirm,
    Pause,
    Quit,
}

pub struct Actions {
    actions: Vec<Action>,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn contains(&self, action: Action) -> bool {
        self.actions.contains(&action)
    }
}
