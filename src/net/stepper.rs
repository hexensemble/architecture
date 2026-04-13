use std::time::{Duration, Instant};

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

        // Drop leftover time to avoid spiral-of-death on long hitches
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

pub struct ServerStepper {
    tick_duration: Duration,
    max_catchup_steps: u32,
    next_tick: Instant,
}

impl ServerStepper {
    pub fn new(fixed_dt: f32, max_catchup_steps: u32) -> Self {
        Self {
            tick_duration: Duration::from_secs_f32(fixed_dt),
            max_catchup_steps,
            next_tick: Instant::now(),
        }
    }

    pub fn wait_and_run<F>(&mut self, mut step: F)
    where
        F: FnMut(),
    {
        // Sleep if ahead
        let now = Instant::now();
        if self.next_tick > now {
            std::thread::sleep(self.next_tick - now);
        }

        // Catch up if behind
        let now = Instant::now();
        let behind = now.duration_since(self.next_tick);
        let ticks_behind = (behind.as_secs_f32() / self.tick_duration.as_secs_f32()) as u32 + 1;
        let steps_to_run = ticks_behind.min(self.max_catchup_steps);
        for _ in 0..steps_to_run {
            step();
            self.next_tick += self.tick_duration;
        }

        // Give up if too far behind
        if ticks_behind > self.max_catchup_steps {
            let skipped = ticks_behind - self.max_catchup_steps;
            log::warn!("[Server Stepper] Falling behind, skipped {} ticks", skipped);
            self.next_tick = Instant::now();
        }
    }
}
