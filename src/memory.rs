const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, offset: usize, length: usize) -> &[u8] {
        let upper_bound = offset + length;
        self.data.get(offset..upper_bound).unwrap()
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) {
        let upper_bound = offset + buffer.len();
        let existing = self.data.get_mut(offset..upper_bound).unwrap();
        for (index, byte) in existing.iter_mut().enumerate() {
            *byte = buffer[index];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn can_create_new_memory() {
        let memory = Memory::new();
        let is_equal = memory.data.iter().eq([0; 4096].iter());
        assert_eq!(is_equal, true);
    }

    #[test]
    fn can_read_memory() {
        let mut memory = Memory::new();
        let offset = 0x13;
        memory.data[offset] = 0xa4;
        memory.data[offset + 1] = 0x22;
        let length = 3;
        let buffer = memory.read(offset, length);
        let upper_bound = offset + length;
        let is_equal = buffer.iter().eq(memory.data[offset..upper_bound].iter());
        assert_eq!(is_equal, true);
    }

    #[test]
    fn can_write_memory() {
        let mut memory = Memory::new();
        let offset = 0x43;
        let buffer = [0x22, 0x43, 0xa1].as_slice();
        let upper_limit = offset + buffer.len();
        memory.write(offset, buffer);
        let is_equal = buffer.iter().eq(memory.data[offset..upper_limit].iter());
        assert_eq!(is_equal, true);
    }
}
