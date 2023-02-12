use crate::font::FONT;
use crate::memory::Memory;
use crate::registers::{RegisterName, Registers};
use crate::utils::convert_to_opcode;
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
            i: 0,
            pc: PROGRAM_START_OFFSET,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
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
        let two_bytes = self.memory.read(self.pc.into(), OPCODE_SIZE as usize);
        let opcode = convert_to_opcode(two_bytes);
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        let t = ((opcode & 0xf000) >> 12) as u8;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let nnn = (opcode & 0x0fff) as usize;
        let nn = (opcode & 0x00ff) as u8;
        let n = (opcode & 0x000f) as u8;

        match t {
            0x0 => match nnn {
                // CLS
                0x0e0 => {}

                // RET
                0x0ee => {
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                }

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            // JP nnn
            0x1 => self.registers.set(RegisterName::I, nnn),

            // CALL nnn
            0x2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.registers.set(RegisterName::PC, nnn);
            }

            // SE Vx, nn
            0x3 => {
                if self.v[x] == nn {
                    self.pc += OPCODE_SIZE;
                }
            }

            // SNE Vx, nn
            0x4 => {
                if self.v[x] != nn {
                    self.pc += OPCODE_SIZE
                }
            }

            // SE Vx, Vy
            0x5 => {
                if self.v[x] == self.v[y] {
                    self.pc += OPCODE_SIZE
                }
            }

            // LD Vx, nn
            0x6 => self.v[x] = nn,

            // ADD Vx, nn
            0x7 => self.v[x] += nn,

            0x8 => match n {
                // LD Vx, Vy
                0x0 => self.v[x] = self.v[y],

                // OR Vx, Vy
                0x1 => self.v[x] |= self.v[y],

                // AND Vx, Vy
                0x2 => self.v[x] &= self.v[y],

                // XOR Vx, Vy
                0x3 => self.v[x] ^= self.v[y],

                // ADD Vx, Vy
                0x4 => {
                    let (result, has_overflown) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = result;
                    if has_overflown {
                        self.v[0xf] = 1;
                    }
                }

                // SUB Vx, Vy
                0x5 => {
                    let (result, has_underflown) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = result;
                    if !has_underflown {
                        self.v[0xf] = 1;
                    }
                }

                // SHR Vx, Vy
                0x6 => {
                    self.v[x] = self.v[y];
                    self.v[0xf] = self.v[x] & 0x1;
                    self.v[x] >>= 1
                }

                // SUBN Vx, Vy
                0x7 => {
                    let (result, has_underflown) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = result;
                    if !has_underflown {
                        self.v[0xf] = 1;
                    }
                }

                // SHL Vx, Vy
                0xe => {
                    self.v[x] = self.v[y];
                    self.v[0xf] = self.v[x] & 0x80;
                    self.v[x] <<= 1;
                }

                // ERR
                _ => panic!("Invalid opcode: {opcode:X}"),
            },

            // LD I, nnn
            0xa => self.registers.set(RegisterName::I, nnn),

            // JP V0, nnn
            0xb => self.pc = nnn + self.v[0] as u16,

            // RND Vx, nn
            0xc => self.v[x] = random::<u8>() & nn,

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
                0x07 => self.v[x] = self.dt,

                // LD Vx, K
                0x0a => {}

                // LD DT, Vx
                0x15 => self.dt = self.v[x],

                // LD ST, Vx
                0x18 => self.st = self.v[x],

                // ADD I, Vx
                0x1e => self.i += self.v[x] as u16,

                // LD F, Vx
                0x29 => {}

                // LD B, Vx
                0x33 => {}

                // LD [I], Vx
                0x55 => {
                    let buffer = &self.v[0..=x];
                    self.memory.write(self.i as usize, buffer);
                }

                // LD Vx, [I]
                0x65 => {
                    let buffer = self.memory.read(self.i as usize, x + 1);
                    for (index, byte) in self.v[0..=x].iter_mut().enumerate() {
                        *byte = buffer[index];
                    }
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
