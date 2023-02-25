mod clock;
mod cpu;
mod font;
mod frame;
pub mod keys;
mod memory;
mod utils;

use clock::Clock;
use cpu::Cpu;
use keys::{Key, KeyState};
use std::fs;
use std::path::Path;

pub struct Chip8 {
    cpu: Cpu,
    pub clock: Clock,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            clock: Clock::new(),
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.frame.get_buffer()
    }

    pub fn load(&mut self, path: &Path) {
        let buffer = fs::read(path).unwrap();
        self.cpu.memory.write(0x200, &buffer)
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn handle_key_press(&mut self, key: Key) {
        self.cpu.update_key_state(key, KeyState::Down);
    }

    pub fn reset(&mut self) {}
}
