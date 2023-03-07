use std::thread;
use std::time::{Duration, Instant};

const ONE_SECOND_IN_MICROSECONDS: u64 = 1_000_000;

pub struct Clock {
    cycle_count: u64,
    instructions_per_second: u64,
    time_per_instruction_microseconds: u64,
    start_time: Option<Instant>,
}

impl Clock {
    pub fn new(instructions_per_second: u64) -> Self {
        Self {
            cycle_count: 0,
            instructions_per_second,
            time_per_instruction_microseconds: ONE_SECOND_IN_MICROSECONDS / instructions_per_second,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.cycle_count = 0;
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.instructions_per_second = instructions_per_second;
    }

    pub fn tick(&mut self) {
        self.cycle_count += 1;

        let actual_elapsed = self
            .start_time
            .expect("the clock hasn't been started")
            .elapsed();
        let expected_elapsed = Duration::from_micros(self.cycle_count * self.time_per_instruction_microseconds);
        if expected_elapsed > actual_elapsed {
            let dt = expected_elapsed - actual_elapsed;
            thread::sleep(dt);
        }
        if self.cycle_count == self.instructions_per_second {
            println!("{:?}", self.start_time.unwrap().elapsed());
            self.start_time = Some(Instant::now());
            self.cycle_count = 0;
        }
    }
}
