mod instructions;
mod timer;

use rand::random;

use crate::display::PixelBuffer;
use crate::font::{FONT, FONT_CHAR_SIZE};
use crate::keypad::{Key, KeyPad, KeyState};
use crate::memory::Memory;
use instructions::Instruction;
use timer::Timer;

const V_REG_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const OPCODE_SIZE: u16 = 2;
const FONT_START_OFFSET: usize = 0;
const PROGRAM_START_OFFSET: u16 = 0x200;

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
    pub ram: Memory,
    pub pixel_buffer: PixelBuffer,
    pub key_pad: KeyPad,
    sound_timer: Timer,
    delay_timer: Timer,
}

impl Cpu {
    pub fn new(cycles_per_timer_decrement: f64) -> Self {
        let mut cpu = Self {
            pc: PROGRAM_START_OFFSET,
            i: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; V_REG_COUNT],
            stack: [0; STACK_SIZE],
            ram: Memory::new(),
            pixel_buffer: PixelBuffer::new(),
            key_pad: KeyPad::new(),
            sound_timer: Timer::new(cycles_per_timer_decrement),
            delay_timer: Timer::new(cycles_per_timer_decrement),
        };

        cpu.ram.load(FONT_START_OFFSET, FONT.as_slice());

        cpu
    }

    pub fn reset(&mut self) {
        self.pc = PROGRAM_START_OFFSET;
        self.i = 0;
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.v = [0; V_REG_COUNT];
        self.stack = [0; STACK_SIZE];
        self.ram = Memory::new();
        self.pixel_buffer = PixelBuffer::new();

        self.delay_timer.stop();
        self.sound_timer.stop();

        self.ram.load(FONT_START_OFFSET, FONT.as_slice());
    }

    pub fn set_timer_speed(&mut self, cycles_per_decrement: f64) {
        self.sound_timer.set_speed(cycles_per_decrement);
        self.delay_timer.set_speed(cycles_per_decrement);
    }

    fn update_timers(&mut self) {
        self.delay_timer.tick();
        self.sound_timer.tick();

        self.dt = self.dt.saturating_sub(self.delay_timer.decrease_by());
        self.st = self.st.saturating_sub(self.sound_timer.decrease_by());

        if self.dt == 0 {
            self.delay_timer.stop();
        }
        if self.st == 0 {
            self.sound_timer.stop();
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let instruction: Instruction = opcode.into();

        match self.execute(instruction) {
            ProgramCounterStatus::Repeat => (),
            ProgramCounterStatus::Next => self.pc += OPCODE_SIZE,
            ProgramCounterStatus::Skip => self.pc += OPCODE_SIZE * 2,
            ProgramCounterStatus::Jump(address) => self.pc = address,
        }

        self.update_timers();
        self.key_pad.reset_released_key_state();
    }

    fn fetch(&self) -> u16 {
        u16::from_be_bytes([self.ram[self.pc as usize], self.ram[self.pc as usize + 1]])
    }

    fn execute(&mut self, instruction: Instruction) -> ProgramCounterStatus {
        let mut program_counter_status = ProgramCounterStatus::Next;

        match instruction {
            Instruction::OpCode00E0 => {
                self.pixel_buffer.clear();
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
                let start_x = self.v[x] as usize;
                let start_y = self.v[y] as usize;

                let sprite = self.ram.read(self.i as usize, n as usize);

                let has_collided = self.pixel_buffer.draw(sprite, (start_x, start_y));

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
