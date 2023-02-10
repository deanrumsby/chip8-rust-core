pub fn convert_to_opcode(two_bytes: &[u8]) -> u16 {
    two_bytes
        .iter()
        .fold(0, |acc, byte| acc * 16 + *byte as u16)
}
