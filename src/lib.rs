#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub use cpu::registers::Registers;
pub use frame::{FrameBuffer, FRAME_HEIGHT, FRAME_WIDTH};
pub use keypad::{Key, KeyState};
use cpu::Cpu;

mod cpu;
mod frame;
mod keypad;

#[cfg(feature = "wasm")]
mod wasm;

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

    /// Returns the width of the frame in pixels.
    pub fn frame_width(&self) -> u32 {
        FRAME_WIDTH as u32
    }

    /// Returns the height of the frame in pixels.
    pub fn frame_height(&self) -> u32 {
        FRAME_HEIGHT as u32
    }

    /// Returns the speed of the virtual machine in instructions per second.
    pub fn speed(&self) -> u32 {
        self.cpu.instructions_per_second
    }

    /// Sets the speed of the virtual machine in instructions per second.
    pub fn set_speed(&mut self, instructions_per_second: u32) {
        self.cpu.set_speed(instructions_per_second);
    }

    /// Loads a program into the virtual machine.
    pub fn load(&mut self, bytes: &[u8]) {
        self.cpu.load_program(bytes);
    }

    /// This will progress the virtual machine by the given time delta.
    /// It takes into account any accumulated time from previous calls that were less than a full cycle.
    /// The time delta given is in microseconds.
    pub fn update(&mut self, time_delta: u32) {
        self.cpu.update(time_delta);
    }

    /// Executes a single cycle of the virtual machine.
    pub fn step(&mut self) {
        self.cpu.step();
    }

    /// Returns a copy of the frame buffer.
    pub fn frame(&self) -> FrameBuffer {
        self.cpu.frame.clone()
    }

    /// Returns a copy of the registers.
    pub fn registers(&self) -> Registers {
        self.cpu.registers.clone()
    }

    /// Sets the registers.
    pub fn set_registers(&mut self, registers: Registers) {
        self.cpu.registers = registers;
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
