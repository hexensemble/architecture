use std::fmt::Debug;

pub trait ActionType: Copy + Clone + PartialEq + Debug {}

pub struct Actions<A: ActionType> {
    actions: Vec<A>,
}

impl<A: ActionType> Actions<A> {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn push(&mut self, action: A) {
        self.actions.push(action);
    }

    pub fn contains(&self, action: A) -> bool {
        self.actions.contains(&action)
    }

    pub fn take(&mut self, action: A) -> bool {
        if let Some(index) = self.actions.iter().position(|a| *a == action) {
            self.actions.remove(index);
            true
        } else {
            false
        }
    }
}
