#[derive(Clone, Copy)]
pub enum Key {
    Key(usize),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KeyState {
    Up,
    Down,
    None,
}
