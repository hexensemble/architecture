#[derive(Debug, Clone)]
pub struct Time {
    delta: f32,
}

impl Time {
    pub fn update(&mut self, delta: f32) {
        self.delta = delta;
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }
}

impl Default for Time {
    fn default() -> Self {
        Self { delta: 0.0 }
    }
}
