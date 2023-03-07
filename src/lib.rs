mod clock;
pub mod cpu;
mod font;

use clock::Clock;
use cpu::{Cpu, Pixel, Key, KeyState};
use std::fs;
use std::path::Path;

const DEFAULT_SPEED: u64 = 700;
const TIMER_FREQUENCY: f64 = 60.0;

pub struct Chip8 {
    pub cpu: Cpu,
    pub clock: Clock,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(DEFAULT_SPEED as f64 / TIMER_FREQUENCY),
            clock: Clock::new(DEFAULT_SPEED),
        }
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.cpu.set_cycles_per_timer_decrement(instructions_per_second as f64 / TIMER_FREQUENCY);
        self.clock.set_speed(instructions_per_second);
    }

    pub fn pixels(&self) -> &[Pixel] {
        self.cpu.pixels()
    }

    pub fn load(&mut self, path: &Path) {
        let buffer = fs::read(path).unwrap();
        self.cpu.load_into_memory(0x200, buffer.as_slice());
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.update_key_pad(key, state);
    }

    pub fn reset(&mut self) {}
}
