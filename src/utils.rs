pub fn concat_bytes(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |acc, byte| acc * 10 + *byte as usize)
}
