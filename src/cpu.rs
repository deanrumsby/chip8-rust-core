mod instructions;

use crate::font::FONT;
use crate::memory::Memory;
use crate::registers::{RegisterName, Registers};
use crate::utils::convert_to_opcode;
use instructions::Instruction;
use rand::random;

const OPCODE_SIZE: usize = 2;
const PROGRAM_START_OFFSET: u16 = 0x200;
const FONT_START_OFFSET: usize = 0;

pub struct Cpu {
    registers: Registers,
    stack: [u16; 16],
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        let registers = Registers::new();
        memory.write(FONT_START_OFFSET, FONT.as_slice());

        Self {
            memory,
            registers,
            stack: [0; 16],
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = Self::decode(opcode);
        self.execute(instruction);
    }

    fn fetch(&self) -> u16 {
        let pc = self.registers.read(RegisterName::PC);
        let two_bytes = self.memory.read(pc, OPCODE_SIZE);
        let opcode = convert_to_opcode(two_bytes);
        opcode
    }

    fn decode(opcode: u16) -> Instruction {
        let op_type = ((opcode & 0xf000) >> 12) as usize;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = (opcode & 0x0fff) as usize;
        let nn = (opcode & 0x00ff) as usize;
        let n = (opcode & 0x000f) as usize;

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

            Instruction::C00EE => {
                let sp = self.registers.read(RegisterName::SP);
                let address = self.stack[sp] as usize;
                self.registers.set(RegisterName::PC, address);
                self.registers.decrement(RegisterName::SP, 1);
            }

            Instruction::C1NNN(nnn) => self.registers.set(RegisterName::I, nnn),

            Instruction::C2NNN(nnn) => {
                self.registers.increment(RegisterName::SP, 1);
                let pc = self.registers.read(RegisterName::PC);
                let sp = self.registers.read(RegisterName::SP);
                self.stack[sp] = pc as u16;
                self.registers.set(RegisterName::PC, nnn);
            }

            Instruction::C3XNN(x, nn) => {
                let vx = self.registers.read(RegisterName::V(x));
                if vx == nn {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            Instruction::C4XNN(x, nn) => {
                let vx = self.registers.read(RegisterName::V(x));
                if vx != nn {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            Instruction::C5XY0(x, y) => {
                let vx = self.registers.read(RegisterName::V(x));
                let vy = self.registers.read(RegisterName::V(y));
                if vx == vy {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            Instruction::C6XNN(x, nn) => self.registers.set(RegisterName::V(x), nn),

            Instruction::C7XNN(x, nn) => {
                self.registers.increment(RegisterName::V(x), nn);
            }

            Instruction::C8XY0(x, y) => {
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(x), vy);
            }

            Instruction::C8XY1(x, y) => {
                let vx = self.registers.read(RegisterName::V(x));
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(x), vx | vy);
            }

            Instruction::C8XY2(x, y) => {
                let vx = self.registers.read(RegisterName::V(x));
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(x), vx & vy);
            }

            Instruction::C8XY3(x, y) => {
                let vx = self.registers.read(RegisterName::V(x));
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(x), vx ^ vy);
            }

            Instruction::C8XY4(x, y) => {
                let vy = self.registers.read(RegisterName::V(y));
                let has_overflown = self.registers.increment(RegisterName::V(x), vy);
                if has_overflown {
                    self.registers.set(RegisterName::V(0xf), 1);
                }
            }

            Instruction::C8XY5(x, y) => {
                let vy = self.registers.read(RegisterName::V(y));
                let has_underflown = self.registers.decrement(RegisterName::V(x), vy);
                if !has_underflown {
                    self.registers.set(RegisterName::V(0xf), 1);
                }
            }

            Instruction::C8XY6(x, y) => {
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(0xf), vy & 0x1);
                self.registers.set(RegisterName::V(x), vy >> 1);
            }

            Instruction::C8XY7(x, y) => {
                let vx = self.registers.read(RegisterName::V(x));
                let has_underflown = self.registers.decrement(RegisterName::V(y), vx);
                if !has_underflown {
                    self.registers.set(RegisterName::V(0xf), 1);
                }
            }

            Instruction::C8XYE(x, y) => {
                let vy = self.registers.read(RegisterName::V(y));
                self.registers.set(RegisterName::V(0xf), vy & 0x80);
                self.registers.set(RegisterName::V(x), vy << 1);
            }

            Instruction::C9XY0(x, y) => {}

            Instruction::CANNN(nnn) => self.registers.set(RegisterName::I, nnn),

            Instruction::CBNNN(nnn) => {
                let v0 = self.registers.read(RegisterName::V(0x0));
                self.registers.set(RegisterName::PC, nnn + v0);
            }

            Instruction::CCXNN(x, nn) => {
                let rand = random::<usize>() & nn;
                self.registers.set(RegisterName::V(x), rand);
            }

            Instruction::CDXYN(x, y, n) => {}

            Instruction::CEX9E(x) => {}

            Instruction::CEXA1(x) => {}

            Instruction::CFX07(x) => {
                let dt = self.registers.read(RegisterName::DT);
                self.registers.set(RegisterName::V(x), dt);
            }

            Instruction::CFX0A(x) => {}

            Instruction::CFX15(x) => {
                let vx = self.registers.read(RegisterName::V(x));
                self.registers.set(RegisterName::DT, vx);
            }

            Instruction::CFX18(x) => {
                let vx = self.registers.read(RegisterName::V(x));
                self.registers.set(RegisterName::ST, vx);
            }

            Instruction::CFX1E(x) => {
                let vx = self.registers.read(RegisterName::V(x));
                self.registers.increment(RegisterName::I, vx);
            }

            Instruction::CFX29(x) => {}

            Instruction::CFX33(x) => {}

            Instruction::CFX55(x) => {
                let vx = self.registers.read(RegisterName::V(x));
                let mut buffer: Vec<u8> = Vec::new();
                for i in 0..=vx {
                    let vi = self.registers.read(RegisterName::V(i));
                    buffer.push(vi as u8);
                }
                let i = self.registers.read(RegisterName::I);
                self.memory.write(i, buffer.as_slice());
            }

            Instruction::CFX65(x) => {
                // let buffer = self.memory.read(self.i as usize, x + 1);
                // for (index, byte) in self.v[0..=x].iter_mut().enumerate() {
                //     *byte = buffer[index];
                // }
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
