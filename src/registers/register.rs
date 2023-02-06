use num_traits::ops::overflowing::{OverflowingAdd, OverflowingSub};

struct Register<T> {
    value: T,
}

impl<T: From<u8>> Register<T> {
    fn new() -> Self {
        Self { value: 0_u8.into() }
    }
}

impl<T> Register<T> {
    fn set(&mut self, value: T) -> () {
        self.value = value;
    }
}

impl<T: Copy> Register<T> {
    fn read(&self) -> T {
        self.value
    }
}

impl<T: OverflowingAdd> Register<T> {
    fn increment(&mut self, value: T) -> bool {
        let (result, has_overflown) = self.value.overflowing_add(&value);
        self.value = result;

        has_overflown
    }
}

impl<T: OverflowingSub> Register<T> {
    fn decrement(&mut self, value: T) -> bool {
        let (result, has_underflown) = self.value.overflowing_sub(&value);
        self.value = result;

        has_underflown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_initial_value() {
        let register: Register<u8> = Register::new();
        assert_eq!(register.value, 0_u8);
    }

    #[test]
    fn can_read_value() {
        let mut register: Register<u16> = Register::new();
        let expected = 0x4e21;
        register.value = expected;
        let value = register.read();
        assert_eq!(value, expected);
    }

    #[test]
    fn can_set_value() {
        let mut register: Register<u16> = Register::new();
        let expected = 0x5a00;
        register.set(expected);
        assert_eq!(register.value, expected);
    }

    #[test]
    fn can_increment_value() {
        let mut register: Register<u8> = Register::new();
        register.value = 0x31;
        register.increment(0x5);
        assert_eq!(register.value, 0x36);
    }

    #[test]
    fn increment_overflow_wraps() {
        let mut register: Register<u16> = Register::new();
        register.value = 0xffff;
        register.increment(3);
        let expected = 0x0002;
        assert_eq!(register.value, expected);
    }

    #[test]
    fn increment_overflow_can_set_false() {
        let mut register: Register<u8> = Register::new();
        register.value = 0xa2;
        let has_overflown = register.increment(2);
        assert_eq!(has_overflown, false);
    }

    #[test]
    fn increment_overflow_can_set_true() {
        let mut register: Register<u32> = Register::new();
        register.value = 0xffffffff;
        let has_overflown = register.increment(4);
        assert_eq!(has_overflown, true);
    }

    #[test]
    fn can_decrement_value() {
        let mut register: Register<u16> = Register::new();
        register.value = 0xde2a;
        register.decrement(5);
        assert_eq!(register.value, 0xde25);
    }

    #[test]
    fn decrement_underflow_wraps() {
        let mut register: Register<u8> = Register::new();
        register.decrement(3);
        assert_eq!(register.value, 0xfd);
    }

    #[test]
    fn decrement_underflow_can_set_false() {
        let mut register: Register<u16> = Register::new();
        register.value = 0x4e2a;
        let has_underflown = register.decrement(5);
        assert_eq!(has_underflown, false);
    }

    #[test]
    fn decrement_underflow_can_set_true() {
        let mut register: Register<u8> = Register::new();
        let has_underflown = register.decrement(3);
        assert_eq!(has_underflown, true);
    }
}
