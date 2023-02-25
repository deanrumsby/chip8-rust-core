pub enum Key {
    Key(usize),
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyState {
    Up,
    Down,
    None,
}
