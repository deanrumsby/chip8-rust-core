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

    pub fn read(&self, offset: usize, size: usize) -> &[u8] {
        &self.data[offset..offset + size]
    }

    pub fn load(&mut self, offset: usize, bytes: &[u8]) {
        let range = offset..offset + bytes.len();
        self.data[range].copy_from_slice(bytes);
    }
}
