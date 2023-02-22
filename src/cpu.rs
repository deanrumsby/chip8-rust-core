mod instructions;

use crate::font::{FONT, FONT_CHAR_SIZE_BYTES};
use crate::frame::Frame;
use crate::memory::Memory;
use crate::utils::concat_bytes;
use instructions::Instruction;
use rand::random;

const OPCODE_SIZE: u16 = 2;
const PROGRAM_START_OFFSET: u16 = 0x200;
const FONT_START_OFFSET: usize = 0;

#[derive(Clone, Copy, PartialEq)]
enum KeyState {
    Up,
    Down,
    None,
}

pub struct Cpu {
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; 16],
    stack: [u16; 16],
    key_state: [KeyState; 16],
    memory: Memory,
    frame: Frame,
}

impl Cpu {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        memory.write(FONT_START_OFFSET, FONT.as_slice());

        Self {
            memory,
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            stack: [0; 16],
            key_state: [KeyState::None; 16],
            frame: Frame::new(),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = Self::decode(opcode);
        match instruction {
            Some(i) => self.execute(i),
            None => panic!("Invalid opcode: {opcode:X}"),
        }
    }

    fn fetch(&self) -> u16 {
        let two_byte_buffer = self.memory.read(self.pc as usize, OPCODE_SIZE as usize);
        let opcode = concat_bytes(two_byte_buffer);
        opcode as u16
    }

    fn decode(opcode: u16) -> Option<Instruction> {
        let op_type = ((opcode & 0xf000) >> 12) as usize;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = (opcode & 0x0fff) as u16;
        let nn = (opcode & 0x00ff) as u8;
        let n = (opcode & 0x000f) as u8;

        match op_type {
            0x0 => match nnn {
                0x0e0 => Some(Instruction::C00E0),
                0x0ee => Some(Instruction::C00EE),
                _ => None,
            },
            0x1 => Some(Instruction::C1NNN(nnn)),
            0x2 => Some(Instruction::C2NNN(nnn)),
            0x3 => Some(Instruction::C3XNN(x, nn)),
            0x4 => Some(Instruction::C4XNN(x, nn)),
            0x5 => Some(Instruction::C5XY0(x, y)),
            0x6 => Some(Instruction::C6XNN(x, nn)),
            0x7 => Some(Instruction::C7XNN(x, nn)),
            0x8 => match n {
                0x0 => Some(Instruction::C8XY0(x, y)),
                0x1 => Some(Instruction::C8XY1(x, y)),
                0x2 => Some(Instruction::C8XY2(x, y)),
                0x3 => Some(Instruction::C8XY3(x, y)),
                0x4 => Some(Instruction::C8XY4(x, y)),
                0x5 => Some(Instruction::C8XY5(x, y)),
                0x6 => Some(Instruction::C8XY6(x, y)),
                0x7 => Some(Instruction::C8XY7(x, y)),
                0xe => Some(Instruction::C8XYE(x, y)),
                _ => None,
            },
            0x9 => Some(Instruction::C9XY0(x, y)),
            0xa => Some(Instruction::CANNN(nnn)),
            0xb => Some(Instruction::CBNNN(nnn)),
            0xc => Some(Instruction::CCXNN(x, nn)),
            0xd => Some(Instruction::CDXYN(x, y, n)),
            0xe => match nn {
                0x9e => Some(Instruction::CEX9E(x)),
                0xa1 => Some(Instruction::CEXA1(x)),
                _ => None,
            },
            0xf => match nn {
                0x07 => Some(Instruction::CFX07(x)),
                0x0a => Some(Instruction::CFX0A(x)),
                0x15 => Some(Instruction::CFX15(x)),
                0x18 => Some(Instruction::CFX18(x)),
                0x1e => Some(Instruction::CFX1E(x)),
                0x29 => Some(Instruction::CFX29(x)),
                0x33 => Some(Instruction::CFX33(x)),
                0x55 => Some(Instruction::CFX55(x)),
                0x65 => Some(Instruction::CFX65(x)),
                _ => None,
            },
            _ => None,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        let mut has_jumped = false;

        match instruction {
            Instruction::C00E0 => self.frame.clear(),

            Instruction::C00EE => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
                has_jumped = true;
            }

            Instruction::C1NNN(nnn) => self.i = nnn,

            Instruction::C2NNN(nnn) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
                has_jumped = true;
            }

            Instruction::C3XNN(x, nn) => {
                if self.v[x] == nn {
                    self.pc += OPCODE_SIZE;
                }
            }

            Instruction::C4XNN(x, nn) => {
                if self.v[x] != nn {
                    self.pc += OPCODE_SIZE;
                }
            }

            Instruction::C5XY0(x, y) => {
                if self.v[x] == self.v[y] {
                    self.pc += OPCODE_SIZE;
                }
            }

            Instruction::C6XNN(x, nn) => {
                self.v[x] = nn;
            }

            Instruction::C7XNN(x, nn) => {
                self.v[x] += nn;
            }

            Instruction::C8XY0(x, y) => self.v[x] = self.v[y],

            Instruction::C8XY1(x, y) => self.v[x] |= self.v[y],

            Instruction::C8XY2(x, y) => self.v[x] &= self.v[y],

            Instruction::C8XY3(x, y) => self.v[x] ^= self.v[y],

            Instruction::C8XY4(x, y) => {
                let (result, has_overflown) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                if has_overflown {
                    self.v[0xf] = 1;
                }
            }

            Instruction::C8XY5(x, y) => {
                let (result, has_underflown) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;
                if !has_underflown {
                    self.v[0xf] = 1;
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
                }
            }

            Instruction::C8XYE(x, y) => {
                self.v[x] = self.v[y];
                self.v[0xf] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }

            Instruction::C9XY0(x, y) => {
                if self.v[x] != self.v[y] {
                    self.pc += OPCODE_SIZE;
                }
            }

            Instruction::CANNN(nnn) => self.i = nnn,

            Instruction::CBNNN(nnn) => {
                self.pc = nnn as u16 + self.v[0] as u16;
                has_jumped = true;
            }

            Instruction::CCXNN(x, nn) => self.v[x] = random::<u8>() & nn,

            Instruction::CDXYN(x, y, n) => {
                let sprite = self.memory.read(self.i as usize, n as usize);
                self.frame.draw_sprite(sprite, (x, y));
            }

            Instruction::CEX9E(x) => {
                let vx = self.v[x];
                match self.key_state[vx as usize] {
                    KeyState::Down => self.pc += OPCODE_SIZE,
                    _ => {}
                }
            }

            Instruction::CEXA1(x) => {
                let vx = self.v[x];
                match self.key_state[vx as usize] {
                    KeyState::Down => {}
                    _ => self.pc += OPCODE_SIZE,
                }
            }

            Instruction::CFX07(x) => self.v[x] = self.dt,

            Instruction::CFX0A(x) => {
                if let Some(key_index) = self.key_state.iter().position(|&key| key == KeyState::Up)
                {
                    self.v[x] = key_index as u8;
                }
            }

            Instruction::CFX15(x) => self.dt = self.v[x],

            Instruction::CFX18(x) => self.st = self.v[x],

            Instruction::CFX1E(x) => self.i += self.v[x] as u16,

            Instruction::CFX29(x) => {
                let nibble = (self.v[x] & 0b1111) as usize;
                self.i = (FONT_START_OFFSET + nibble * FONT_CHAR_SIZE_BYTES) as u16;
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

        if !has_jumped {
            self.pc += OPCODE_SIZE;
        }
    }
}
