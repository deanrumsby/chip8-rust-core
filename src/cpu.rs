use alloc::borrow::ToOwned;
use nanorand::{Rng, WyRand};

use crate::frame::FrameBuffer;
use crate::keypad::{Key, KeyPad, KeyState};
use font::{FONT, FONT_CHAR_SIZE};
use instructions::Instruction;
use memory::Memory;

mod font;
mod instructions;
mod memory;

const V_REG_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const OPCODE_SIZE: u16 = 2;
const FONT_START_OFFSET: usize = 0;
const PROGRAM_START_OFFSET: u16 = 0x200;
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
    pc: u16,
    i: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; V_REG_COUNT],
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
            pc: PROGRAM_START_OFFSET,
            i: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; V_REG_COUNT],
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
        self.pc = PROGRAM_START_OFFSET;
        self.i = 0;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.v = [0; V_REG_COUNT];
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
            ProgramCounterStatus::Next => self.pc += OPCODE_SIZE,
            ProgramCounterStatus::Skip => self.pc += OPCODE_SIZE * 2,
            ProgramCounterStatus::Jump(address) => self.pc = address,
        }
    }

    fn step_timer(&mut self, timer: Timer) {
        let (register, accumulator) = match timer {
            Timer::Delay => (&mut self.dt, &mut self.dt_time_accumulator),
            Timer::Sound => (&mut self.st, &mut self.st_time_accumulator),
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
                .read(self.pc as usize, 2)
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
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }

            Instruction::OpCode1NNN(nnn) => {
                program_counter_status = ProgramCounterStatus::Jump(nnn);
            }

            Instruction::OpCode2NNN(nnn) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                program_counter_status = ProgramCounterStatus::Jump(nnn);
            }

            Instruction::OpCode3XNN(x, nn) => {
                if self.v[x] == nn {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode4XNN(x, nn) => {
                if self.v[x] != nn {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode5XY0(x, y) => {
                if self.v[x] == self.v[y] {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCode6XNN(x, nn) => {
                self.v[x] = nn;
            }

            Instruction::OpCode7XNN(x, nn) => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }

            Instruction::OpCode8XY0(x, y) => {
                self.v[x] = self.v[y];
            }

            Instruction::OpCode8XY1(x, y) => {
                self.v[x] |= self.v[y];
            }

            Instruction::OpCode8XY2(x, y) => {
                self.v[x] &= self.v[y];
            }

            Instruction::OpCode8XY3(x, y) => {
                self.v[x] ^= self.v[y];
            }

            Instruction::OpCode8XY4(x, y) => {
                let (result, has_overflown) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                if has_overflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XY5(x, y) => {
                let (result, has_underflown) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;
                if !has_underflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XY6(x, y) => {
                self.v[x] = self.v[y];
                self.v[0xf] = self.v[x] & 0x1;
                self.v[x] >>= 1
            }

            Instruction::OpCode8XY7(x, y) => {
                let (result, has_underflown) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = result;
                if !has_underflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::OpCode8XYE(x, y) => {
                self.v[x] = self.v[y];
                self.v[0xf] = (self.v[x] >> 7) & 0x1;
                self.v[x] <<= 1;
            }

            Instruction::OpCode9XY0(x, y) => {
                if self.v[x] != self.v[y] {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCodeANNN(nnn) => {
                self.i = nnn;
            }

            Instruction::OpCodeBNNN(nnn) => {
                program_counter_status = ProgramCounterStatus::Jump(nnn + self.v[0] as u16);
            }

            Instruction::OpCodeCXNN(x, nn) => {
                self.v[x] = self.rng.generate::<u8>() & nn;
            }

            Instruction::OpCodeDXYN(x, y, n) => {
                let start_x = self.v[x] as usize;
                let start_y = self.v[y] as usize;

                let sprite = self.ram.read(self.i as usize, n as usize);

                let has_collided = self.frame.draw(sprite, (start_x, start_y));

                self.v[0xf] = if has_collided { 1 } else { 0 };
            }

            Instruction::OpCodeEX9E(x) => {
                let key: Key = self.v[x].into();
                if self.key_pad.get(key) == KeyState::Pressed {
                    program_counter_status = ProgramCounterStatus::Skip;
                }
            }

            Instruction::OpCodeEXA1(x) => {
                let key: Key = self.v[x].into();
                match self.key_pad.get(key) {
                    KeyState::Pressed => {}
                    _ => program_counter_status = ProgramCounterStatus::Skip,
                }
            }

            Instruction::OpCodeFX07(x) => {
                self.v[x] = self.dt;
            }

            Instruction::OpCodeFX0A(x) => {
                let key = self.key_pad.find_released_key();
                match key {
                    Some(key) => self.v[x] = key.into(),
                    None => program_counter_status = ProgramCounterStatus::Repeat,
                }
            }

            Instruction::OpCodeFX15(x) => {
                self.dt = self.v[x];
            }

            Instruction::OpCodeFX18(x) => {
                self.st = self.v[x];
            }

            Instruction::OpCodeFX1E(x) => {
                self.i = self.i.wrapping_add(self.v[x] as u16);
            }

            Instruction::OpCodeFX29(x) => {
                let nibble = (self.v[x] & 0b1111) as usize;
                self.i = (FONT_START_OFFSET + (nibble * FONT_CHAR_SIZE)) as u16;
            }

            Instruction::OpCodeFX33(x) => {
                let vx = self.v[x];
                let units = vx % 10;
                let tens = (vx / 10) % 10;
                let hundreds = (vx / 100) % 10;
                self.ram.load(self.i as usize, &[hundreds, tens, units]);
            }

            Instruction::OpCodeFX55(x) => {
                let buffer = &self.v[0..=x].to_owned();
                self.ram.load(self.i as usize, buffer);
            }

            Instruction::OpCodeFX65(x) => {
                let buffer = self.ram.read(self.i as usize, x + 1);
                self.v[0..=x].copy_from_slice(buffer);
            }
        }
        program_counter_status
    }
}
