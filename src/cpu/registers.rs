#[cfg(feature = "wasm")]
use {
    crate::alloc::string::ToString,
    serde::{Deserialize, Serialize},
    tsify::Tsify,
};

const PROGRAM_START_OFFSET: u16 = 0x200;
const V_REG_COUNT: usize = 16;

#[cfg_attr(feature = "wasm", derive(Tsify, Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone)]
pub struct Registers {
    pub pc: u16,
    pub i: u16,
    pub sp: u8,
    pub dt: u8,
    pub st: u8,
    pub v: [u8; V_REG_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: PROGRAM_START_OFFSET,
            i: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; V_REG_COUNT],
        }
    }
}
