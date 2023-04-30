mod cpu;
mod display;
mod font;
mod keypad;
mod memory;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use cpu::Cpu;
pub use display::{Pixel, PIXELS_HEIGHT, PIXELS_WIDTH};
pub use keypad::{Key, KeyState};


const DEFAULT_SPEED: u64 = 700;
const DEFAULT_FRAME_RATE: u64 = 60;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Chip8 {
    instructions_per_second: u64,
    frame_rate: u64,
    cpu: Cpu,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Chip8 {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            instructions_per_second: DEFAULT_SPEED,
            frame_rate: DEFAULT_FRAME_RATE,
            cpu: Cpu::new(DEFAULT_SPEED),
        }
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.instructions_per_second = instructions_per_second;
        self.cpu.set_speed(instructions_per_second);
    }

    pub fn set_frame_rate(&mut self, frame_rate: u64) {
        self.frame_rate = frame_rate;
    }

    pub fn pixels(&self) -> &[Pixel] {
        self.cpu.pixel_buffer.pixels()
    }

    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.ram.load(0x200, bytes);
    }

    pub fn emulate(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.cpu.step();
        }
    }

    pub fn emulate_frame(&mut self, frames: usize) {
        let cycles_per_frame = self.instructions_per_second / self.frame_rate;
        for _ in 0..frames {
            self.emulate(cycles_per_frame);
        }
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad.set(key, state);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
