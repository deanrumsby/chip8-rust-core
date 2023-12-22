use alloc::borrow::ToOwned;
use nanorand::{Rng, WyRand};

use crate::frame::FrameBuffer;
use crate::keypad::{Key, KeyPad, KeyState};
use font::{FONT, FONT_CHAR_SIZE};
use instructions::Instruction;
use memory::Memory;
use registers::Registers;

pub mod registers;
mod font;
mod instructions;
mod memory;

const STACK_SIZE: usize = 16;
const OPCODE_SIZE: u16 = 2;
const FONT_START_OFFSET: usize = 0;
const ONE_SECOND_IN_MICRO_SECONDS: u32 = 1_000_000;
const DEFAULT_INSTRUCTIONS_PER_SECOND: u32 = 700;
const TIMER_STEP_THRESHOLD_MICRO_SECONDS: u32 = 16_666;
const PROGRAM_START: usize = 0x200;

enum ProgramCounterStatus {
    Repeat,
    Next,
    Skip,
    Jump(u16),
}

enum Timer {
    Delay,
    Sound,
}

pub struct Cpu {
    rng: WyRand,
    cpu_time_accumulator: u32,
    instructions_per_second: u32,
    micro_seconds_per_instruction: u32,
    pub registers: Registers, 
    stack: [u16; STACK_SIZE],
    pub ram: Memory,
    pub frame: FrameBuffer,
    pub key_pad: KeyPad,
    st_time_accumulator: u32,
    dt_time_accumulator: u32,
}

impl Cpu {
    pub fn new(seed: u32) -> Self {
        let mut cpu = Self {
            rng: WyRand::new_seed(seed.into()),
            cpu_time_accumulator: 0,
            instructions_per_second: 0,
            micro_seconds_per_instruction: 0,
            registers: Registers::new(),
            stack: [0; STACK_SIZE],
            ram: Memory::new(),
            frame: FrameBuffer::new(),
            key_pad: KeyPad::new(),
            st_time_accumulator: 0,
            dt_time_accumulator: 0,
        };

        cpu.set_speed(DEFAULT_INSTRUCTIONS_PER_SECOND);
        cpu.ram.load(FONT_START_OFFSET, FONT.as_slice());
        cpu
    }

    pub fn load_program(&mut self, bytes: &[u8]) {
        self.ram.load(PROGRAM_START, bytes);
    }

    pub fn reset(&mut self) {
        self.cpu_time_accumulator = 0;
        self.registers = Registers::new();
        self.stack = [0; STACK_SIZE];
        self.ram = Memory::new();
        self.dt_time_accumulator = 0;
        self.st_time_accumulator = 0;
        self.frame.clear();
        self.ram.load(FONT_START_OFFSET, FONT.as_slice());
    }

    pub fn set_speed(&mut self, instructions_per_second: u32) {
        self.instructions_per_second = instructions_per_second;
        self.micro_seconds_per_instruction = ONE_SECOND_IN_MICRO_SECONDS / instructions_per_second;
    }

    pub fn update(&mut self, time_delta: u32) {
        let total_time_accumulated = self.cpu_time_accumulator + time_delta;
        let instructions_to_emulate = total_time_accumulated / self.micro_seconds_per_instruction;
        for _ in 0..instructions_to_emulate {
            self.step();
        }
        let time_progressed = instructions_to_emulate * self.micro_seconds_per_instruction;
        self.cpu_time_accumulator = total_time_accumulated - time_progressed;
    }
    
    pub fn step(&mut self) {
        self.step_instruction();
        self.step_timer(Timer::Delay);
        self.step_timer(Timer::Sound);
        self.key_pad.reset_released_keys();
    }

    fn step_instruction(&mut self) {
        let opcode = self.fetch();
        let instruction: Instruction = opcode.into();

        match self.execute(instruction) {
            ProgramCounterStatus::Repeat => (),
            ProgramCounterStatus::Next => self.registers.pc += OPCODE_SIZE,
            ProgramCounterStatus::Skip => self.registers.pc += OPCODE_SIZE * 2,
            ProgramCounterStatus::Jump(address) => self.registers.pc = address,
        }
    }

    fn step_timer(&mut self, timer: Timer) {
        let (register, accumulator) = match timer {
            Timer::Delay => (&mut self.registers.dt, &mut self.dt_time_accumulator),
            Timer::Sound => (&mut self.registers.st, &mut self.st_time_accumulator),
        };

        if *register > 0 {
            let accumulated_time = *accumulator + self.micro_seconds_per_instruction;
            if accumulated_time >= TIMER_STEP_THRESHOLD_MICRO_SECONDS {
                *accumulator = accumulated_time - TIMER_STEP_THRESHOLD_MICRO_SECONDS;
                *register = register.saturating_sub(1);
            } else {
                *accumulator = accumulated_time;
            }
        } else {
            *accumulator = 0;
        }
    }

    fn fetch(&self) -> u16 {
        u16::from_be_bytes(
            self.ram
                .read(self.registers.pc as usize, 2)
                .try_into()
                .expect("failed to fetch instruction"),
        )
    }

    fn execute(&mut self, instruction: Instruction) -> ProgramCounterStatus {
        let mut program_counter_status = ProgramCounterStatus::Next;

        match instruction {
            Instruction::OpCode00E0 => {
                self.frame.clear();
            }

            Instruction::OpCode00EE => {
                self.registers.pc = self.stack[self.registers.sp as usize];
                self.registers.sp -= 1;
            }

            Instruction::OpCode1NNN(nnn) => {
                program_counter_status = ProgramCounterStatus::Jump(nnn);
            }

            Instruction::OpCode2NNN(nnn) => {
                self.registers.sp += 1;
                self.stack[self.registers.sp as usize] = self.registers.pc;
                program_counter_status = ProgramCounterStatus::Jump(nnn);
            }

            Instruction::OpCode3XNN(x, nn) => {
                if self.registers.v[x] == nn {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode4XNN(x, nn) => {
                if self.registers.v[x] != nn {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode5XY0(x, y) => {
                if self.registers.v[x] == self.registers.v[y] {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode6XNN(x, nn) => {
                self.registers.v[x] = nn;
            }

            Instruction::OpCode7XNN(x, nn) => {
                self.registers.v[x] = self.registers.v[x].wrapping_add(nn);
            }

            Instruction::OpCode8XY0(x, y) => {
                self.registers.v[x] = self.registers.v[y];
            }

            Instruction::OpCode8XY1(x, y) => {
                self.registers.v[x] |= self.registers.v[y];
            }

            Instruction::OpCode8XY2(x, y) => {
                self.registers.v[x] &= self.registers.v[y];
            }

            Instruction::OpCode8XY3(x, y) => {
                self.registers.v[x] ^= self.registers.v[y];
            }

            Instruction::OpCode8XY4(x, y) => {
                let (result, has_overflown) = self.registers.v[x].overflowing_add(self.registers.v[y]);
                self.registers.v[x] = result;
                if has_overflown {
                    self.registers.v[0xf] = 1;
                } else {
                    self.registers.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XY5(x, y) => {
                let (result, has_underflown) = self.registers.v[x].overflowing_sub(self.registers.v[y]);
                self.registers.v[x] = result;
                if !has_underflown {
                    self.registers.v[0xf] = 1;
                } else {
                    self.registers.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XY6(x, y) => {
                self.registers.v[x] = self.registers.v[y];
                self.registers.v[0xf] = self.registers.v[x] & 0x1;
                self.registers.v[x] >>= 1
            }

            Instruction::OpCode8XY7(x, y) => {
                let (result, has_underflown) = self.registers.v[y].overflowing_sub(self.registers.v[x]);
                self.registers.v[x] = result;
                if !has_underflown {
                    self.registers.v[0xf] = 1;
                } else {
                    self.registers.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XYE(x, y) => {
                self.registers.v[x] = self.registers.v[y];
                self.registers.v[0xf] = (self.registers.v[x] >> 7) & 0x1;
                self.registers.v[x] <<= 1;
            }

            Instruction::OpCode9XY0(x, y) => {
                if self.registers.v[x] != self.registers.v[y] {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCodeANNN(nnn) => {
                self.registers.i = nnn;
            }

            Instruction::OpCodeBNNN(nnn) => {
                program_counter_status = ProgramCounterStatus::Jump(nnn + self.registers.v[0] as u16);
            }

            Instruction::OpCodeCXNN(x, nn) => {
                self.registers.v[x] = self.rng.generate::<u8>() & nn;
            }

            Instruction::OpCodeDXYN(x, y, n) => {
                let start_x = self.registers.v[x] as usize;
                let start_y = self.registers.v[y] as usize;

                let sprite = self.ram.read(self.registers.i as usize, n as usize);

                let has_collided = self.frame.draw(sprite, (start_x, start_y));

                self.registers.v[0xf] = if has_collided { 1 } else { 0 };
            }

            Instruction::OpCodeEX9E(x) => {
                let key: Key = self.registers.v[x].into();
                if self.key_pad.get(key) == KeyState::Pressed {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCodeEXA1(x) => {
                let key: Key = self.registers.v[x].into();
                match self.key_pad.get(key) {
                    KeyState::Pressed => {}
                    _ => program_counter_status = ProgramCounterStatus::Skip,
                }
            }

            Instruction::OpCodeFX07(x) => {
                self.registers.v[x] = self.registers.dt;
            }

            Instruction::OpCodeFX0A(x) => {
                let key = self.key_pad.find_released_key();
                match key {
                    Some(key) => self.registers.v[x] = key.into(),
                    None => program_counter_status = ProgramCounterStatus::Repeat,
                }
            }

            Instruction::OpCodeFX15(x) => {
                self.registers.dt = self.registers.v[x];
            }

            Instruction::OpCodeFX18(x) => {
                self.registers.st = self.registers.v[x];
            }

            Instruction::OpCodeFX1E(x) => {
                self.registers.i = self.registers.i.wrapping_add(self.registers.v[x] as u16);
            }

            Instruction::OpCodeFX29(x) => {
                let nibble = (self.registers.v[x] & 0b1111) as usize;
                self.registers.i = (FONT_START_OFFSET + (nibble * FONT_CHAR_SIZE)) as u16;
            }

            Instruction::OpCodeFX33(x) => {
                let vx = self.registers.v[x];
                let units = vx % 10;
                let tens = (vx / 10) % 10;
                let hundreds = (vx / 100) % 10;
                self.ram.load(self.registers.i as usize, &[hundreds, tens, units]);
            }

            Instruction::OpCodeFX55(x) => {
                let buffer = &self.registers.v[0..=x].to_owned();
                self.ram.load(self.registers.i as usize, buffer);
            }

            Instruction::OpCodeFX65(x) => {
                let buffer = self.ram.read(self.registers.i as usize, x + 1);
                self.registers.v[0..=x].copy_from_slice(buffer);
            }
        }
        program_counter_status
    }
}
