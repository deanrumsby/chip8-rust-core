const CYCLES_PER_DECREMENT: f32 = 700.0 / 60.0;
const CYCLES_PER_SECOND: f32 = 700.0;

pub struct Timer {
    is_active: bool,
    cycle_count: f32,
    decrement_count: f32,
    pub should_decrease: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            is_active: false,
            cycle_count: 0.0,
            decrement_count: 0.0,
            should_decrease: false,
        }
    }

    pub fn start(&mut self) {
        self.cycle_count = 0.0;
        self.decrement_count = 0.0;
        self.is_active = true;
        self.should_decrease = false;
    }

    pub fn tick(&mut self) {
        self.should_decrease = false;

        if !self.is_active {
            return;
        }
        self.cycle_count += 1.0;
        let count_threshold = CYCLES_PER_DECREMENT * (self.decrement_count + 1.0);
        if self.cycle_count >= count_threshold {
            self.decrement_count += 1.0;
            self.should_decrease = true;
        }
        if self.cycle_count == CYCLES_PER_SECOND {
            self.cycle_count = 0.0;
            self.decrement_count = 0.0;
        }
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }
}
