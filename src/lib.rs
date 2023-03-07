mod clock;
pub mod cpu;
mod font;

use clock::Clock;
use cpu::{Cpu, Pixel, Key, KeyState};
use std::fs;
use std::path::Path;

pub struct Chip8 {
    pub cpu: Cpu,
    pub clock: Clock,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            clock: Clock::new(),
        }
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
