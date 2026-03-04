pub struct FixedStepper {
    accumulator: f32,
    max_steps_per_frame: u32,
}

impl FixedStepper {
    pub fn new(max_steps_per_frame: u32) -> Self {
        Self {
            accumulator: 0.0,
            max_steps_per_frame,
        }
    }

    pub fn run_steps<F>(&mut self, fixed_dt: f32, mut step: F)
    where
        F: FnMut(),
    {
        let mut steps = 0;

        while self.accumulator >= fixed_dt && steps < self.max_steps_per_frame {
            step();
            self.subtract_time(fixed_dt);
            steps += 1;
        }

        // Drop leftover time to avoid spiral-of-death on long hitches.
        if steps == self.max_steps_per_frame {
            self.accumulator = 0.0
        }
    }

    pub fn add_time(&mut self, dt: f32) {
        self.accumulator += dt;
    }

    pub fn subtract_time(&mut self, dt: f32) {
        self.accumulator -= dt;
    }
}
