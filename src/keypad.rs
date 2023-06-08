#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub const KEY_COUNT: usize = 16;

pub struct KeyPad {
    state: [KeyState; KEY_COUNT],
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq)]
pub enum KeyState {
    Released,
    Pressed,
    None,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
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

    pub fn get(&self, key: Key) -> KeyState {
        self.state[u8::from(key) as usize]
    }

    pub fn set(&mut self, key: Key, state: KeyState) {
        self.state[u8::from(key) as usize] = state;
    }

    pub fn find_released_key(&self) -> Option<Key> {
        for (i, key) in self.state.iter().enumerate() {
            if *key == KeyState::Released {
                return Some((i as u8).into());
            }
        }
        None
    }

    pub fn reset_released_keys(&mut self) {
        for key in self.state.iter_mut() {
            if *key == KeyState::Released {
                *key = KeyState::None;
            }
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
