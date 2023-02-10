mod cpu;
mod memory;
mod utils;

use cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Self { cpu: Cpu::new() }
    }

    pub fn load(&mut self, program: &[u8]) {}

    pub fn step(&self) {
        self.cpu.step();
    }

    pub fn reset(&mut self) {}
}
