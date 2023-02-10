pub struct Cpu {
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    v: [u8; 16],
}

impl Cpu {
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
}
