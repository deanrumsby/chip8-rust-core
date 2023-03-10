use std::ops::{Index, IndexMut};

pub const KEY_COUNT: usize = 16;

pub struct KeyPad {
    state: [KeyState; KEY_COUNT],
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyState {
    Released,
    Pressed,
    None,
}

#[derive(Clone, Copy)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl KeyPad {
    pub fn new() -> Self {
        Self {
            state: [KeyState::None; KEY_COUNT],
        }
    }

    pub fn find_released_key(&self) -> Option<Key> {
        for (i, key) in self.state.iter().enumerate() {
            if *key == KeyState::Released {
                return Some((i as u8).into());
            }
        }
        None
    }

    pub fn reset_released_key_state(&mut self) {
        for key in self.state.iter_mut() {
            if *key == KeyState::Released {
                *key = KeyState::None;
            }
        }
    }
}

impl Index<Key> for KeyPad {
    type Output = KeyState;

    fn index(&self, key: Key) -> &Self::Output {
        match key {
            Key::Key0 => &self.state[0x0],
            Key::Key1 => &self.state[0x1],
            Key::Key2 => &self.state[0x2],
            Key::Key3 => &self.state[0x3],
            Key::Key4 => &self.state[0x4],
            Key::Key5 => &self.state[0x5],
            Key::Key6 => &self.state[0x6],
            Key::Key7 => &self.state[0x7],
            Key::Key8 => &self.state[0x8],
            Key::Key9 => &self.state[0x9],
            Key::KeyA => &self.state[0xA],
            Key::KeyB => &self.state[0xB],
            Key::KeyC => &self.state[0xC],
            Key::KeyD => &self.state[0xD],
            Key::KeyE => &self.state[0xE],
            Key::KeyF => &self.state[0xF],
        }
    }
}

impl IndexMut<Key> for KeyPad {
    fn index_mut(&mut self, key: Key) -> &mut Self::Output {
        match key {
            Key::Key0 => &mut self.state[0x0],
            Key::Key1 => &mut self.state[0x1],
            Key::Key2 => &mut self.state[0x2],
            Key::Key3 => &mut self.state[0x3],
            Key::Key4 => &mut self.state[0x4],
            Key::Key5 => &mut self.state[0x5],
            Key::Key6 => &mut self.state[0x6],
            Key::Key7 => &mut self.state[0x7],
            Key::Key8 => &mut self.state[0x8],
            Key::Key9 => &mut self.state[0x9],
            Key::KeyA => &mut self.state[0xA],
            Key::KeyB => &mut self.state[0xB],
            Key::KeyC => &mut self.state[0xC],
            Key::KeyD => &mut self.state[0xD],
            Key::KeyE => &mut self.state[0xE],
            Key::KeyF => &mut self.state[0xF],
        }
    }
}

impl From<u8> for Key {
    fn from(key: u8) -> Self {
        match key {
            0x0 => Key::Key0,
            0x1 => Key::Key1,
            0x2 => Key::Key2,
            0x3 => Key::Key3,
            0x4 => Key::Key4,
            0x5 => Key::Key5,
            0x6 => Key::Key6,
            0x7 => Key::Key7,
            0x8 => Key::Key8,
            0x9 => Key::Key9,
            0xA => Key::KeyA,
            0xB => Key::KeyB,
            0xC => Key::KeyC,
            0xD => Key::KeyD,
            0xE => Key::KeyE,
            0xF => Key::KeyF,
            _ => panic!("Invalid key: {}", key),
        }
    }
}

impl From<Key> for u8 {
    fn from(key: Key) -> Self {
        match key {
            Key::Key0 => 0x0,
            Key::Key1 => 0x1,
            Key::Key2 => 0x2,
            Key::Key3 => 0x3,
            Key::Key4 => 0x4,
            Key::Key5 => 0x5,
            Key::Key6 => 0x6,
            Key::Key7 => 0x7,
            Key::Key8 => 0x8,
            Key::Key9 => 0x9,
            Key::KeyA => 0xA,
            Key::KeyB => 0xB,
            Key::KeyC => 0xC,
            Key::KeyD => 0xD,
            Key::KeyE => 0xE,
            Key::KeyF => 0xF,
        }
    }
}
