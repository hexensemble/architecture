#[derive(Debug, Clone)]
pub struct Time {
    delta: f32,
    elapsed: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.delta = delta;
        self.elapsed += delta;
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }
}
