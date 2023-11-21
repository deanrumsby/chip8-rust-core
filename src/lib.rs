#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "wasm")]
use {
    wasm_bindgen::prelude::*,
    js_sys::Uint8ClampedArray,
};

use cpu::Cpu;
pub use frame::{FRAME_HEIGHT, FRAME_WIDTH};
pub use keypad::{Key, KeyState};

mod cpu;
mod frame;
mod keypad;

/// Struct representing the Chip-8 virtual machine.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Chip8 {
    cpu: Cpu,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Chip8 {
    /// Creates a new Chip-8 virtual machine.
    /// We use a seed to initialize the random number generator for portability across environments.
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(seed: u32) -> Self {
        Self {
            cpu: Cpu::new(seed),
        }
    }

    /// Returns a reference to the frame buffer.
    #[cfg(not(feature = "wasm"))]
    pub fn frame(&self) -> &[u8] {
        self.cpu.frame.buffer.as_slice()
    }

    /// Returns a Uint8ClampedArray copy of the frame buffer.
    #[cfg(feature = "wasm")]
    pub fn frame(&self) -> Uint8ClampedArray {
        Uint8ClampedArray::from(self.cpu.frame.buffer.as_slice())
    }

    /// Returns the width of the frame in pixels.
    /// Useful when accessing the code as a WASM module, as we can't access constants from JS.
    pub fn frame_width(&self) -> u32 {
        FRAME_WIDTH as u32
    }

    /// Returns the height of the frame in pixels.
    /// Useful when accessing the code as a WASM module, as we can't access constants from JS.
    pub fn frame_height(&self) -> u32 {
        FRAME_HEIGHT as u32
    }

    /// Sets the speed of the virtual machine.
    pub fn set_speed(&mut self, instructions_per_second: u32) {
        self.cpu.set_speed(instructions_per_second);
    }

    /// Loads a program into the virtual machine.
    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.load_program(bytes);
    }

    /// Updates the virtual machine's state.
    /// The time delta is in microseconds.
    /// The cpu will execute instructions until the time delta is reached, plus any remaining time from the previous update.
    pub fn update(&mut self, time_delta: u32) {
        self.cpu.update(time_delta);
    }

    /// Executes a single cycle of the virtual machine.
    /// This will execute a single instruction and update the delay and sound timers.
    pub fn step(&mut self) {
        self.cpu.step();
    }

    /// Passes a key event to the virtual machine.
    pub fn handle_key_event(&mut self, key: Key, state: KeyState) {
        self.cpu.key_pad.set(key, state);
    }

    /// Resets the virtual machine.
    /// The seed for the random number generator is not reset.
    /// All registers, the stack, timers, ram and the frame buffer are reset.
    /// The font is reloaded... However any program that was in memory is cleared, and will need
    /// to be loaded again.
    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
