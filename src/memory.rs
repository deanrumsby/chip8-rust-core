pub struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 4096] }
    }
}
