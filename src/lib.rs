mod clock;
mod cpu;
mod display;
mod font;
mod keypad;

use std::fs;
use std::path::Path;

use clock::Clock;
use cpu::Cpu;
pub use display::{Pixel, PIXELS_HEIGHT, PIXELS_WIDTH};
pub use keypad::{Key, KeyState};

const DEFAULT_SPEED: u64 = 700;
const TIMER_FREQUENCY: f64 = 60.0;

pub struct Chip8 {
    cpu: Cpu,
    clock: Clock,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(DEFAULT_SPEED as f64 / TIMER_FREQUENCY),
            clock: Clock::new(DEFAULT_SPEED),
        }
    }

    pub fn start(&mut self) {
        self.clock.start();
    }

    pub fn stop(&mut self) {
        self.clock.stop();
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.cpu
            .set_timer_speed(instructions_per_second as f64 / TIMER_FREQUENCY);
        self.clock.set_speed(instructions_per_second);
    }

    pub fn pixels(&self) -> &[Pixel] {
        self.cpu.pixel_buffer.pixels()
    }

    pub fn load(&mut self, path: &Path) {
        let buffer = fs::read(path).unwrap();
        self.cpu.load_into_memory(0x200, buffer.as_slice());
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad[key] = state;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.stop();
    }
}
