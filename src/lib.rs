mod cpu;
mod memory;

use cpu::Cpu;
use memory::Memory;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, program: &[u8]) {}

    pub fn step(&mut self) {}

    pub fn reset(&mut self) {}
}
