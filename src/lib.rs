mod cpu;
mod frame;
mod keypad;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use cpu::Cpu;
pub use frame::{FRAME_HEIGHT, FRAME_WIDTH};
pub use keypad::{Key, KeyState};

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

    /// Sets the frame buffer to use for rendering.
    /// The frame buffer must be a slice of length 64 * 32 * 4 (8192 bytes).
    /// Each pixel is represented by 4 bytes (RGBA).
    /// This replaces the frame buffer that was previously set (or the default internal frame buffer).
    pub fn set_frame_buffer(&mut self, frame_buffer: &mut [u8]) {
        self.cpu.set_frame_buffer(frame_buffer);
    }

    /// Returns a pointer to the frame buffer. 
    /// Useful when accessing the code as a WASM module, to avoid copying the frame buffer to JS
    /// land.
    /// See `examples/browser` and `examples/browser-bundler` for examples.
    pub fn frame_buffer_mut_ptr(&mut self) -> *mut u8 {
        self.cpu.frame.mut_ptr()
    }

    /// Returns the length of the frame buffer.
    /// Useful when accessing the code as a WASM module, to avoid copying the frame buffer to JS
    /// land.
    /// See `examples/browser` and `examples/browser-bundler` for examples.
    pub fn frame_buffer_len(&self) -> usize {
        self.cpu.frame.len()
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
    pub fn reset(&mut self, seed: u64) {
        self.cpu.reset(seed);
    }
}
