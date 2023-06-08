mod cpu;
mod font;
mod frame;
mod keypad;
mod memory;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use js_sys::{Uint8Array, Uint8ClampedArray};

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
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
        }
    }

    pub fn start(&mut self, timestamp: u64) {
        self.cpu.start(timestamp);
    }

    pub fn set_speed(&mut self, instructions_per_second: u64) {
        self.cpu.set_speed(instructions_per_second);
    }

    #[cfg(not(feature = "wasm"))]
    pub fn frame_buffer(&self) -> &[u8] {
        self.cpu.frame.frame_buffer()
    }

    #[cfg(feature = "wasm")]
    pub fn frame_buffer(&self) -> Uint8ClampedArray {
        Uint8ClampedArray::from(self.cpu.frame.frame_buffer())
    }

    #[cfg(not(feature = "wasm"))]
    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.load_program(bytes);
    }

    #[cfg(feature = "wasm")]
    pub fn load(&mut self, bytes: Uint8Array) {
        self.cpu.load_program(bytes.to_vec().as_slice());
    }

    pub fn emulate(&mut self, timestamp: u64) {
        self.cpu.emulate(timestamp);
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad.set(key, state);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
