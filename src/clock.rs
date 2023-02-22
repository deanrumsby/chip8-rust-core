use std::thread;
use std::time::{Duration, Instant};

const INSTRUCTIONS_PER_SECOND: u64 = 700;
const TIME_PER_INSTRUCTION_NS: u64 = 1_000_000_000 / INSTRUCTIONS_PER_SECOND;

pub struct Clock {
    cycle_count: u64,
    start_time: Option<Instant>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            cycle_count: 0,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.cycle_count = 0;
    }

    pub fn tick(&mut self) {
        let actual_elapsed = self
            .start_time
            .expect("the clock hasn't been started")
            .elapsed();
        let expected_elapsed = Duration::from_nanos(self.cycle_count * TIME_PER_INSTRUCTION_NS);
        let dt = expected_elapsed - actual_elapsed;
        if dt > Duration::ZERO {
            thread::sleep(dt);
        }
        if self.cycle_count == INSTRUCTIONS_PER_SECOND {
            self.start_time = Some(Instant::now());
            self.cycle_count = 1;
        } else {
            self.cycle_count += 1;
        }
    }
}
