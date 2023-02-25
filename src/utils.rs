pub fn concat_bytes(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .fold(0, |acc, byte| (acc << 8) + *byte as usize)
}

pub fn is_bit_set(value: u8, bit: usize) -> bool {
    (value & (1 << bit)) != 0
}
