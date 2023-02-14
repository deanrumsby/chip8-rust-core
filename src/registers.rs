use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum RegisterName {
    I,
    PC,
    SP,
    DT,
    ST,
    V(usize),
}

#[derive(PartialEq, Eq)]
enum RegisterValue {
    U8(u8),
    U16(u16),
}

pub struct Registers {
    map: HashMap<RegisterName, RegisterValue>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                (RegisterName::I, RegisterValue::U16(0)),
                (RegisterName::PC, RegisterValue::U16(0)),
                (RegisterName::SP, RegisterValue::U8(0)),
                (RegisterName::DT, RegisterValue::U8(0)),
                (RegisterName::ST, RegisterValue::U8(0)),
                (RegisterName::V(0x0), RegisterValue::U8(0)),
                (RegisterName::V(0x1), RegisterValue::U8(0)),
                (RegisterName::V(0x2), RegisterValue::U8(0)),
                (RegisterName::V(0x3), RegisterValue::U8(0)),
                (RegisterName::V(0x4), RegisterValue::U8(0)),
                (RegisterName::V(0x5), RegisterValue::U8(0)),
                (RegisterName::V(0x6), RegisterValue::U8(0)),
                (RegisterName::V(0x7), RegisterValue::U8(0)),
                (RegisterName::V(0x8), RegisterValue::U8(0)),
                (RegisterName::V(0x9), RegisterValue::U8(0)),
                (RegisterName::V(0xa), RegisterValue::U8(0)),
                (RegisterName::V(0xb), RegisterValue::U8(0)),
                (RegisterName::V(0xc), RegisterValue::U8(0)),
                (RegisterName::V(0xd), RegisterValue::U8(0)),
                (RegisterName::V(0xe), RegisterValue::U8(0)),
                (RegisterName::V(0xf), RegisterValue::U8(0)),
            ]),
        }
    }

    pub fn read(&self, name: RegisterName) -> usize {
        let register_value = self.map.get(&name).unwrap();
        match register_value {
            RegisterValue::U16(current_value) => *current_value as usize,
            RegisterValue::U8(current_value) => *current_value as usize,
        }
    }

    pub fn set(&mut self, name: RegisterName, value: usize) {
        let register_value = self.map.get_mut(&name).unwrap();
        match register_value {
            RegisterValue::U16(current_value) => *current_value = value as u16,
            RegisterValue::U8(current_value) => *current_value = value as u8,
        }
    }

    pub fn increment(&mut self, name: RegisterName, value: usize) -> bool {
        let register_value = self.map.get_mut(&name).unwrap();
        match register_value {
            RegisterValue::U16(current_value) => {
                let (result, has_overflown) = current_value.overflowing_add(value as u16);
                *current_value = result;
                has_overflown
            }
            RegisterValue::U8(current_value) => {
                let (result, has_overflown) = current_value.overflowing_add(value as u8);
                *current_value = result;
                has_overflown
            }
        }
    }

    pub fn decrement(&mut self, name: RegisterName, value: usize) -> bool {
        let register_value = self.map.get_mut(&name).unwrap();
        match register_value {
            RegisterValue::U16(current_value) => {
                let (result, has_underflown) = current_value.overflowing_sub(value as u16);
                *current_value = result;
                has_underflown
            }
            RegisterValue::U8(current_value) => {
                let (result, has_underflown) = current_value.overflowing_sub(value as u8);
                *current_value = result;
                has_underflown
            }
        }
    }

    // pub fn are_equal(&self, register_a: RegisterName, register_b: RegisterName) -> bool {
    //     let register_a_value = self.map.get(&register_a).unwrap();
    //     let register_b_value = self.map.get(&register_b).unwrap();
    //     register_a_value == register_b_value
    // }

    // pub fn is_equal(&self, register: RegisterName, value: usize) -> bool {
    //     let register_value = self.map.get(&register).unwrap();
    //     match register_value {
    //         RegisterValue::U16(current_value) => *current_value as usize == value,
    //         RegisterValue::U8(current_value) => *current_value as usize == value,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_a_register() {
        let registers = Registers::new();
        let value = registers.read(RegisterName::PC);
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

    #[test]
    fn is_equal_determines_true_correctly() {
        let mut registers = Registers::new();
        let test_value = 0x3a;
        registers.set(RegisterName::V(3), test_value);
        registers.set(RegisterName::V(8), test_value);
        assert_eq!(
            registers.are_equal(RegisterName::V(3), RegisterName::V(8)),
            true
        );
    }

    #[test]
    fn is_equal_determines_false_correctly() {
        let mut registers = Registers::new();
        let test_value_1 = 0x3a;
        let test_value_2 = 0x55;
        registers.set(RegisterName::V(3), test_value_1);
        registers.set(RegisterName::V(8), test_value_2);
        assert_eq!(
            registers.are_equal(RegisterName::V(3), RegisterName::V(8)),
            false
        );
    }
}

// OpTest::new(opcode).with_reg(reg, value).with_mem(offset, buffer).expect_reg(reg, value).run()
