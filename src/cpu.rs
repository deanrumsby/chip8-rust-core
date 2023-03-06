mod instructions;
mod timer;

use crate::font::{FONT, FONT_CHAR_SIZE};
use crate::frame::Frame;
use crate::keys::{Key, KeyState};
use crate::memory::Memory;
use instructions::Instruction;
use rand::random;
use timer::Timer;

use std::fmt;

const OPCODE_SIZE: u16 = 2;
const FONT_START_OFFSET: usize = 0;

pub struct Cpu {
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; 16],
    stack: [u16; 16],
    key_state: [KeyState; 16],
    sound_timer: Timer,
    delay_timer: Timer,
    opcode: u16,
    pub memory: Memory,
    pub frame: Frame,
    pub redraw: bool,
}

enum ProgramCounterAction {
    Repeat,
    Next,
    Skip,
    Jump(u16),
}

impl Cpu {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        memory.write(FONT_START_OFFSET, FONT.as_slice());

        Self {
            memory,
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            stack: [0; 16],
            key_state: [KeyState::None; 16],
            sound_timer: Timer::new(),
            delay_timer: Timer::new(),
            opcode: 0,
            frame: Frame::new(),
            redraw: false,
        }
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
        self.opcode = opcode;
        let instruction = Instruction::try_from(opcode).unwrap();
        
        match self.execute(instruction) {
            ProgramCounterAction::Repeat => (),
            ProgramCounterAction::Next => self.pc += OPCODE_SIZE,
            ProgramCounterAction::Skip => self.pc += OPCODE_SIZE * 2,
            ProgramCounterAction::Jump(address) => self.pc = address,
        }

        self.delay_timer.tick();
        self.sound_timer.tick();
    }

    fn fetch(&self) -> u16 {
        let two_byte_buffer = self.memory.read(self.pc as usize, OPCODE_SIZE as usize);
        let opcode = u16::from_be_bytes([two_byte_buffer[0], two_byte_buffer[1]]);
        opcode as u16
    }

    fn execute(&mut self, instruction: Instruction) -> ProgramCounterAction {
        let mut program_counter_action = ProgramCounterAction::Next;
        
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
            Instruction::C00E0 => {
                self.frame.clear();
                self.redraw = true;
            }

            Instruction::C00EE => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }

            Instruction::C1NNN(nnn) => {
                program_counter_action = ProgramCounterAction::Jump(nnn);
            }

            Instruction::C2NNN(nnn) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                program_counter_action = ProgramCounterAction::Jump(nnn);
            }

            Instruction::C3XNN(x, nn) => {
                if self.v[x] == nn {
                    program_counter_action = ProgramCounterAction::Skip;
                }
            }

            Instruction::C4XNN(x, nn) => {
                if self.v[x] != nn {
                    program_counter_action = ProgramCounterAction::Skip;
                }
            }

            Instruction::C5XY0(x, y) => {
                if self.v[x] == self.v[y] {
                    program_counter_action = ProgramCounterAction::Skip;
                }
            }

            Instruction::C6XNN(x, nn) => {
                self.v[x] = nn;
            }

            Instruction::C7XNN(x, nn) => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }

            Instruction::C8XY0(x, y) => {
                self.v[x] = self.v[y];
            }

            Instruction::C8XY1(x, y) => {
                self.v[x] |= self.v[y];
            }

            Instruction::C8XY2(x, y) => {
                self.v[x] &= self.v[y];
            }

            Instruction::C8XY3(x, y) => {
                self.v[x] ^= self.v[y];
            }

            Instruction::C8XY4(x, y) => {
                let (result, has_overflown) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                if has_overflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::C8XY5(x, y) => {
                let (result, has_underflown) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;
                if !has_underflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::C8XY6(x, y) => {
                self.v[x] = self.v[y];
                self.v[0xf] = self.v[x] & 0x1;
                self.v[x] >>= 1
            }

            Instruction::C8XY7(x, y) => {
                let (result, has_underflown) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = result;
                if !has_underflown {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
            }

            Instruction::C8XYE(x, y) => {
                self.v[x] = self.v[y];
                self.v[0xf] = (self.v[x] >> 7) & 0x1;
                self.v[x] <<= 1;
            }

            Instruction::C9XY0(x, y) => {
                if self.v[x] != self.v[y] {
                    program_counter_action = ProgramCounterAction::Skip;
                }
            }

            Instruction::CANNN(nnn) => {
                self.i = nnn;
            }

            Instruction::CBNNN(nnn) => {
                program_counter_action = ProgramCounterAction::Jump(nnn + self.v[0] as u16);
            }

            Instruction::CCXNN(x, nn) => {
                self.v[x] = random::<u8>() & nn;
            }

            Instruction::CDXYN(x, y, n) => {
                let sprite = self.memory.read(self.i as usize, n as usize);
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

            Instruction::CEX9E(x) => {
                let vx = self.v[x] as usize;
                match self.key_state[vx] {
                    KeyState::Down => program_counter_action = ProgramCounterAction::Skip,
                    _ => {}
                }
            }

            Instruction::CEXA1(x) => {
                let vx = self.v[x] as usize;
                match self.key_state[vx] {
                    KeyState::Down => {}
                    _ => program_counter_action = ProgramCounterAction::Skip,
                }
            }

            Instruction::CFX07(x) => {
                self.v[x] = self.dt;
            }

            Instruction::CFX0A(x) => {
                match self
                    .key_state
                    .iter()
                    .position(|&state| state == KeyState::Up)
                {
                    Some(key_index) => self.v[x] = key_index as u8,
                    None => program_counter_action = ProgramCounterAction::Repeat,
                }
            }

            Instruction::CFX15(x) => {
                self.dt = self.v[x];
                self.delay_timer.start();
            }

            Instruction::CFX18(x) => {
                self.st = self.v[x];
                self.sound_timer.start();
            }

            Instruction::CFX1E(x) => {
                self.i = self.i.wrapping_add(self.v[x] as u16);
            }

            Instruction::CFX29(x) => {
                let nibble = (self.v[x] & 0b1111) as usize;
                self.i = (FONT_START_OFFSET + (nibble * FONT_CHAR_SIZE)) as u16;
            }

            Instruction::CFX33(x) => {
                let vx = self.v[x];
                let units = vx % 10;
                let tens = (vx / 10) % 10;
                let hundreds = (vx / 100) % 10;
                self.memory
                    .write(self.i as usize, [hundreds, tens, units].as_slice());
            }

            Instruction::CFX55(x) => {
                let buffer = &self.v[0..=x];
                self.memory.write(self.i as usize, buffer);
            }

            Instruction::CFX65(x) => {
                let buffer = self.memory.read(self.i as usize, x + 1);
                for (index, byte) in self.v[0..=x].iter_mut().enumerate() {
                    *byte = buffer[index];
                }
            }
        }

        program_counter_action
    }
}
