mod cpu;
mod frame;
mod keypad;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use cpu::Cpu;
pub use frame::{FRAME_HEIGHT, FRAME_WIDTH};
pub use keypad::{Key, KeyState};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Chip8 {
    cpu: Cpu,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Chip8 {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(seed: u64) -> Self {
        Self {
            cpu: Cpu::new(seed),
        }
    }

    pub fn set_frame_buffer(&mut self, frame_buffer: &mut [u8]) {
        self.cpu.set_frame_buffer(frame_buffer);
    }

    pub fn frame_buffer_mut_ptr(&mut self) -> *mut u8 {
        self.cpu.frame.mut_ptr()
    }

    pub fn frame_buffer_len(&self) -> usize {
        self.cpu.frame.len()
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.cpu.set_speed(instructions_per_second);
    }

    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.load_program(bytes);
    }

    pub fn update(&mut self, time_delta: u64) {
        self.cpu.update(time_delta);
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad.set(key, state);
    }

    pub fn reset(&mut self, seed: u64) {
        self.cpu.reset(seed);
    }
}
