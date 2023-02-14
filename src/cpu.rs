mod instructions;

use crate::font::FONT;
use crate::memory::Memory;
use crate::utils::convert_to_opcode;
use instructions::Instruction;
use rand::random;

const OPCODE_SIZE: u16 = 2;
const PROGRAM_START_OFFSET: u16 = 0x200;
const FONT_START_OFFSET: usize = 0;

pub struct Cpu {
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; 16],
    stack: [u16; 16],
    memory: Memory,
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
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = Self::decode(opcode);
        self.execute(instruction);
    }

    fn fetch(&self) -> u16 {
        let two_bytes = self.memory.read(self.pc.into(), OPCODE_SIZE as usize);
        let opcode = convert_to_opcode(two_bytes);
        opcode
    }

    fn decode(opcode: u16) -> Instruction {
        let op_type = ((opcode & 0xf000) >> 12) as usize;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = (opcode & 0x0fff) as u16;
        let nn = (opcode & 0x00ff) as u8;
        let n = (opcode & 0x000f) as u8;

        match op_type {
            0x0 => match nnn {
                0x0e0 => Instruction::C00E0,
                0x0ee => Instruction::C00EE,
                _ => panic!("Invalid opcode: {opcode:X}"),
            },
            0x1 => Instruction::C1NNN(nnn),
            0x2 => Instruction::C2NNN(nnn),
            0x3 => Instruction::C3XNN(x, nn),
            0x4 => Instruction::C4XNN(x, nn),
            0x5 => Instruction::C5XY0(x, y),
            0x6 => Instruction::C6XNN(x, nn),
            0x7 => Instruction::C7XNN(x, nn),
            0x8 => match n {
                0x0 => Instruction::C8XY0(x, y),
                0x1 => Instruction::C8XY1(x, y),
                0x2 => Instruction::C8XY2(x, y),
                0x3 => Instruction::C8XY3(x, y),
                0x4 => Instruction::C8XY4(x, y),
                0x5 => Instruction::C8XY5(x, y),
                0x6 => Instruction::C8XY6(x, y),
                0x7 => Instruction::C8XY7(x, y),
                0xe => Instruction::C8XYE(x, y),
                _ => panic!("Invalid opcode: {opcode:X}"),
            },
            0x9 => Instruction::C9XY0(x, y),
            0xa => Instruction::CANNN(nnn),
            0xb => Instruction::CBNNN(nnn),
            0xc => Instruction::CCXNN(x, nn),
            0xd => Instruction::CDXYN(x, y, n),
            0xe => match nn {
                0x9e => Instruction::CEX9E(x),
                0xa1 => Instruction::CEXA1(x),
                _ => panic!("Invalid opcode: {opcode:X}"),
            },
            0xf => match nn {
                0x07 => Instruction::CFX07(x),
                0x0a => Instruction::CFX0A(x),
                0x15 => Instruction::CFX15(x),
                0x18 => Instruction::CFX18(x),
                0x1e => Instruction::CFX1E(x),
                0x29 => Instruction::CFX29(x),
                0x33 => Instruction::CFX33(x),
                0x55 => Instruction::CFX55(x),
                0x65 => Instruction::CFX65(x),
                _ => panic!("Invalid opcode: {opcode:X}"),
            },
            _ => panic!("Invalid opcode: {opcode:X}"),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::C00E0 => {}

            Instruction::C00EE => {}

            Instruction::C1NNN(nnn) => self.i = nnn,

            Instruction::C2NNN(nnn) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
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

            Instruction::C9XY0(x, y) => {}

            Instruction::CANNN(nnn) => self.i = nnn,

            Instruction::CBNNN(nnn) => self.pc = nnn as u16 + self.v[0] as u16,

            Instruction::CCXNN(x, nn) => self.v[x] = random::<u8>() & nn,

            Instruction::CDXYN(x, y, n) => {}

            Instruction::CEX9E(x) => {}

            Instruction::CEXA1(x) => {}

            Instruction::CFX07(x) => self.v[x] = self.dt,

            Instruction::CFX0A(x) => {}

            Instruction::CFX15(x) => self.dt = self.v[x],

            Instruction::CFX18(x) => self.st = self.v[x],

            Instruction::CFX1E(x) => self.i += self.v[x] as u16,

            Instruction::CFX29(x) => {}

            Instruction::CFX33(x) => {}

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
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    struct OpcodeTest {
        expected: Cpu,
        state: Cpu,
    }

    impl OpcodeTest {}
}
