pub enum RegisterName {
    I,
    PC,
    SP,
    DT,
    ST,
    V(usize),
}

pub struct Registers {
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; 16],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
        }
    }

    pub fn read(&self, name: RegisterName) -> usize {
        match name {
            RegisterName::I => self.i as usize,
            RegisterName::PC => self.pc as usize,
            RegisterName::SP => self.sp as usize,
            RegisterName::DT => self.dt as usize,
            RegisterName::ST => self.st as usize,
            RegisterName::V(index) => self.v[index] as usize,
        }
    }

    pub fn set(&mut self, name: RegisterName, value: usize) {
        match name {
            RegisterName::I => self.i = value as u16,
            RegisterName::PC => self.pc = value as u16,
            RegisterName::SP => self.sp = value as u8,
            RegisterName::DT => self.dt = value as u8,
            RegisterName::ST => self.st = value as u8,
            RegisterName::V(index) => self.v[index] = value as u8,
        }
    }

    pub fn increment(&mut self, name: RegisterName, value: usize) -> bool {
        match name {
            RegisterName::I => {
                let (result, has_overflown) = self.i.overflowing_add(value as u16);
                self.i = result;
                has_overflown
            }
            RegisterName::PC => {
                let (result, has_overflown) = self.pc.overflowing_add(value as u16);
                self.pc = result;
                has_overflown
            }
            RegisterName::SP => {
                let (result, has_overflown) = self.sp.overflowing_add(value as u8);
                self.sp = result;
                has_overflown
            }
            RegisterName::DT => {
                let (result, has_overflown) = self.dt.overflowing_add(value as u8);
                self.dt = result;
                has_overflown
            }
            RegisterName::ST => {
                let (result, has_overflown) = self.st.overflowing_add(value as u8);
                self.st = result;
                has_overflown
            }
            RegisterName::V(index) => {
                let (result, has_overflown) = self.v[index].overflowing_add(value as u8);
                self.v[index] = result;
                has_overflown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_a_named_register() {
        let registers = Registers::new();
        let value = registers.read(RegisterName::PC);
        assert_eq!(value, 0);
    }

    #[test]
    fn can_read_a_v_register() {
        let registers = Registers::new();
        let value = registers.read(RegisterName::V(0x8));
        assert_eq!(value, 0);
    }

    #[test]
    fn can_read_a_set_value() {
        let mut registers = Registers::new();
        let test_value = 0x2121;
        registers.set(RegisterName::I, test_value);
        let register_value = registers.read(RegisterName::I);
        assert_eq!(register_value, test_value);
    }
}

// OpTest::new(opcode).with_reg(reg, value).with_mem(offset, buffer).expect_reg(reg, value).run()
