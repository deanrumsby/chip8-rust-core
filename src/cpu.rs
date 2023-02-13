use crate::font::FONT;
use crate::memory::Memory;
use crate::registers::{RegisterName, Registers};
use crate::utils::convert_to_opcode;
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
        self.execute(opcode);
    }

    fn fetch(&self) -> u16 {
        let pc = self.registers.read(RegisterName::PC);
        let two_bytes = self.memory.read(pc, OPCODE_SIZE);
        let opcode = convert_to_opcode(two_bytes);
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        let t = ((opcode & 0xf000) >> 12) as u8;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = (opcode & 0x0fff) as usize;
        let nn = (opcode & 0x00ff) as usize;
        let n = (opcode & 0x000f) as u8;

        match t {
            0x0 => match nnn {
                // CLS
                0x0e0 => {}

                // RET
                0x0ee => {
                    let sp = self.registers.read(RegisterName::SP);
                    let address = self.stack[sp] as usize;
                    self.registers.set(RegisterName::PC, address);
                    self.registers.decrement(RegisterName::SP, 1);
                }

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            // JP nnn
            0x1 => self.registers.set(RegisterName::I, nnn),

            // CALL nnn
            0x2 => {
                self.registers.increment(RegisterName::SP, 1);
                let pc = self.registers.read(RegisterName::PC);
                let sp = self.registers.read(RegisterName::SP);
                self.stack[sp] = pc as u16;
                self.registers.set(RegisterName::PC, nnn);
            }

            // SE Vx, nn
            0x3 => {
                let vx = self.registers.read(RegisterName::V(x));
                if vx == nn {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            // SNE Vx, nn
            0x4 => {
                let vx = self.registers.read(RegisterName::V(x));
                if vx != nn {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            // SE Vx, Vy
            0x5 => {
                let vx = self.registers.read(RegisterName::V(x));
                let vy = self.registers.read(RegisterName::V(y));
                if vx == vy {
                    self.registers.increment(RegisterName::PC, OPCODE_SIZE);
                }
            }

            // LD Vx, nn
            0x6 => self.registers.set(RegisterName::V(x), nn),

            // ADD Vx, nn
            0x7 => {
                self.registers.increment(RegisterName::V(x), nn);
            }

            0x8 => match n {
                // LD Vx, Vy
                0x0 => {
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(x), vy);
                }

                // OR Vx, Vy
                0x1 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(x), vx | vy);
                }

                // AND Vx, Vy
                0x2 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(x), vx & vy);
                }

                // XOR Vx, Vy
                0x3 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(x), vx ^ vy);
                }

                // ADD Vx, Vy
                0x4 => {
                    let vy = self.registers.read(RegisterName::V(y));
                    let has_overflown = self.registers.increment(RegisterName::V(x), vy);
                    if has_overflown {
                        self.registers.set(RegisterName::V(0xf), 1);
                    }
                }

                // SUB Vx, Vy
                0x5 => {
                    let vy = self.registers.read(RegisterName::V(y));
                    let has_underflown = self.registers.decrement(RegisterName::V(x), vy);
                    if !has_underflown {
                        self.registers.set(RegisterName::V(0xf), 1);
                    }
                }

                // SHR Vx, Vy
                0x6 => {
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(0xf), vy & 0x1);
                    self.registers.set(RegisterName::V(x), vy >> 1);
                }

                // SUBN Vx, Vy
                0x7 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    let has_underflown = self.registers.decrement(RegisterName::V(y), vx);
                    if !has_underflown {
                        self.registers.set(RegisterName::V(0xf), 1);
                    }
                }

                // SHL Vx, Vy
                0xe => {
                    let vy = self.registers.read(RegisterName::V(y));
                    self.registers.set(RegisterName::V(0xf), vy & 0x80);
                    self.registers.set(RegisterName::V(x), vy << 1);
                }

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            // LD I, nnn
            0xa => self.registers.set(RegisterName::I, nnn),

            // JP V0, nnn
            0xb => {
                let v0 = self.registers.read(RegisterName::V(0x0));
                self.registers.set(RegisterName::PC, nnn + v0);
            }
            // RND Vx, nn
            0xc => {
                let rand = random::<usize>() & nn;
                self.registers.set(RegisterName::V(x), rand);
            }

            // DRW Vx, Vy, n
            0xd => {}

            0xe => match nn {
                // SKP Vx
                0x9e => {}

                // SKNP Vx
                0xa1 => {}

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            0xf => match nn {
                // LD Vx, DT
                0x07 => {
                    let dt = self.registers.read(RegisterName::DT);
                    self.registers.set(RegisterName::V(x), dt);
                }

                // LD Vx, K
                0x0a => {}

                // LD DT, Vx
                0x15 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    self.registers.set(RegisterName::DT, vx);
                }

                // LD ST, Vx
                0x18 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    self.registers.set(RegisterName::ST, vx);
                }

                // ADD I, Vx
                0x1e => {
                    let vx = self.registers.read(RegisterName::V(x));
                    self.registers.increment(RegisterName::I, vx);
                }

                // LD F, Vx
                0x29 => {}

                // LD B, Vx
                0x33 => {}

                // LD [I], Vx
                0x55 => {
                    let vx = self.registers.read(RegisterName::V(x));
                    let mut buffer: Vec<u8> = Vec::new();
                    for i in 0..=vx {
                        let vi = self.registers.read(RegisterName::V(i));
                        buffer.push(vi as u8);
                    }
                    let i = self.registers.read(RegisterName::I);
                    self.memory.write(i, buffer.as_slice());
                }

                // LD Vx, [I]
                0x65 => {
                    // let buffer = self.memory.read(self.i as usize, x + 1);
                    // for (index, byte) in self.v[0..=x].iter_mut().enumerate() {
                    //     *byte = buffer[index];
                    // }
                }

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            // ERR
            _ => panic!("Invalid opcode: {opcode:X}"),
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
