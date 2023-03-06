mod instructions;
mod timer;

use crate::font::{FONT, FONT_CHAR_SIZE};
use crate::frame::Frame;
use crate::keys::{Key, KeyState};
use instructions::Instruction;
use rand::random;
use timer::Timer;

const V_REG_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const KEY_COUNT: usize = 16;
const OPCODE_SIZE: u16 = 2;
const FONT_START_OFFSET: usize = 0;
const PROGRAM_START_OFFSET: u16 = 0x200;
const MEMORY_SIZE: usize = 4096;

enum ProgramCounterStatus {
    Repeat,
    Next,
    Skip,
    Jump(u16),
}

pub struct Cpu {
    pc: u16,
    i: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; V_REG_COUNT],
    stack: [u16; STACK_SIZE],
    ram: [u8; MEMORY_SIZE],
    key_state: [KeyState; KEY_COUNT],
    sound_timer: Timer,
    delay_timer: Timer,
    pub frame: Frame,
    pub redraw: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut interpreter = Self {
            pc: PROGRAM_START_OFFSET,
            i: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; V_REG_COUNT],
            stack: [0; STACK_SIZE],
            ram: [0; MEMORY_SIZE],
            key_state: [KeyState::None; KEY_COUNT],
            sound_timer: Timer::new(),
            delay_timer: Timer::new(),
            frame: Frame::new(),
            redraw: false,
        };

        interpreter.load_into_memory(FONT_START_OFFSET, FONT.as_slice());

        interpreter
    }

    pub fn load_into_memory(&mut self, offset: usize, bytes: &[u8]) {
        let range = offset..offset + bytes.len();
        self.ram[range].copy_from_slice(bytes);
    }

    fn read_from_memory(&self, offset: usize, size: usize) -> &[u8] {
        &self.ram[offset..offset + size]
    }

    pub fn update_key_state(&mut self, key: Key, state: KeyState) {
        match key {
            Key::Key(index) => self.key_state[index] = state,
        }
    }

    fn reset_key_up_state(&mut self) {
        self.key_state = self.key_state.map(|state| match state {
            KeyState::Up => KeyState::None,
            other => other,
        })
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        self.redraw = false;
        let instruction = Instruction::try_from(opcode).unwrap();
        
        match self.execute(instruction) {
            ProgramCounterStatus::Repeat => (),
            ProgramCounterStatus::Next => self.pc += OPCODE_SIZE,
            ProgramCounterStatus::Skip => self.pc += OPCODE_SIZE * 2,
            ProgramCounterStatus::Jump(address) => self.pc = address,
        }

        self.delay_timer.tick();
        self.sound_timer.tick();
    }

    fn fetch(&self) -> u16 {
        u16::from_be_bytes([
            self.ram[self.pc as usize],
            self.ram[self.pc as usize + 1],
        ])
    }

    fn execute(&mut self, instruction: Instruction) -> ProgramCounterStatus {
        let mut program_counter_status = ProgramCounterStatus::Next;
        
        if self.sound_timer.should_decrease && self.st > 0 {
            self.st -= 1;
            if self.st == 0 {
                self.sound_timer.stop();
            }
        }

        if self.delay_timer.should_decrease && self.dt > 0 {
            self.dt -= 1;
            if self.dt == 0 {
                self.delay_timer.stop();
            }
        }

        match instruction {
            Instruction::OpCode00E0 => {
                self.frame.clear();
                self.redraw = true;
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
                self.v[x] = random::<u8>() & nn;
            }

            Instruction::OpCodeDXYN(x, y, n) => {
                let sprite = &self.ram[self.i as usize..self.i as usize + n as usize];
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;
                let has_collision = self.frame.draw_sprite(sprite, (vx, vy));
                if has_collision {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
                self.redraw = true;
            }

            Instruction::OpCodeEX9E(x) => {
                let vx = self.v[x] as usize;
                match self.key_state[vx] {
                    KeyState::Down => program_counter_status = ProgramCounterStatus::Skip,
                    _ => {}
                }
            }

            Instruction::OpCodeEXA1(x) => {
                let vx = self.v[x] as usize;
                match self.key_state[vx] {
                    KeyState::Down => {}
                    _ => program_counter_status = ProgramCounterStatus::Skip,
                }
            }

            Instruction::OpCodeFX07(x) => {
                self.v[x] = self.dt;
            }

            Instruction::OpCodeFX0A(x) => {
                match self
                    .key_state
                    .iter()
                    .position(|&state| state == KeyState::Up)
                {
                    Some(key_index) => self.v[x] = key_index as u8,
                    None => program_counter_status = ProgramCounterStatus::Repeat,
                }
            }

            Instruction::OpCodeFX15(x) => {
                self.dt = self.v[x];
                self.delay_timer.start();
            }

            Instruction::OpCodeFX18(x) => {
                self.st = self.v[x];
                self.sound_timer.start();
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
                self.load_into_memory(self.i as usize, &[hundreds, tens, units]);
            }

            Instruction::OpCodeFX55(x) => {
                let buffer = &self.v[0..=x].to_owned();
                self.load_into_memory(self.i as usize, buffer);
            }

            Instruction::OpCodeFX65(x) => {
                let buffer = &self.read_from_memory(self.i as usize, x + 1).to_owned();
                self.v[0..=x].copy_from_slice(buffer);
            }
        }

        program_counter_status
    }
}
