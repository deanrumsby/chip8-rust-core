const TIMER_FREQUENCY: f64 = 60.0;

pub struct Timer {
    is_active: bool,
    cycles_per_decrement: f64,
    cycle_count: u64,
    decrement_count: u64,
    should_decrease: bool,
}

impl Timer {
    pub fn new(cpu_speed: f64) -> Self {
        Self {
            is_active: false,
            cycles_per_decrement: cpu_speed / TIMER_FREQUENCY,
            cycle_count: 0,
            decrement_count: 0,
            should_decrease: false,
        }
    }

    pub fn start(&mut self) {
        self.cycle_count = 0;
        self.decrement_count = 0;
        self.is_active = true;
        self.should_decrease = false;
    }

    pub fn set_speed(&mut self, cpu_speed: u64) {
        self.cycles_per_decrement = cpu_speed as f64 / TIMER_FREQUENCY;
    }

    pub fn decrease_by(&self) -> u8 {
        if self.should_decrease {
            (1.0 / self.cycles_per_decrement).ceil() as u8
        } else {
            0
        }
    }

    pub fn tick(&mut self) {
        if !self.is_active {
            return;
        }
        self.should_decrease = false;
        self.cycle_count += 1;
        let count_threshold = self.cycles_per_decrement * (self.decrement_count + 1) as f64;
        if self.cycle_count as f64 >= count_threshold {
            self.decrement_count += 1;
            self.should_decrease = true;
        }
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }
}
