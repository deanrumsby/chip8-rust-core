mod cpu;
mod frame;
mod font;
mod keypad;
mod memory;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use cpu::Cpu;
pub use keypad::{Key, KeyState};
pub use frame::{PIXEL_ON, PIXEL_OFF, FRAME_WIDTH, FRAME_HEIGHT};

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

    #[cfg(not(feature = "wasm"))]
    pub fn frame(&self) -> &[u8] {
        self.cpu.frame.frame()
    }

    #[cfg(feature = "wasm")]
    pub fn frame(&self) -> js_sys::Uint8ClampedArray {
        js_sys::Uint8ClampedArray::from(self.cpu.frame.frame())
    }

    #[cfg(not(feature = "wasm"))]
    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.ram.load(0x200, bytes);
    }

    #[cfg(feature = "wasm")]
    pub fn load(&mut self, bytes: js_sys::Uint8Array) {
        self.cpu.ram.load(0x200, bytes.to_vec().as_slice());
    }


    pub fn emulate(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.cpu.step();
        }
    }

    pub fn emulate_frame(&mut self, frames: u32) {
        let cycles_per_frame = self.instructions_per_second / self.frame_rate;
        for _ in 0..frames {
            self.emulate(cycles_per_frame as u32);
        }
    }

    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad.set(key, state);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
