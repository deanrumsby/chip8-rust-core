pub const FRAME_WIDTH: usize = 64;
pub const FRAME_HEIGHT: usize = 32;
pub const PIXEL_ON: [u8; 4] = [u8::MAX, u8::MAX, u8::MAX, u8::MAX];
pub const PIXEL_OFF: [u8; 4] = [u8::MIN, u8::MIN, u8::MIN, u8::MAX];

const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT;
const BYTES_PER_PIXEL: usize = 4;
const BYTES_PER_ROW: usize = FRAME_WIDTH * BYTES_PER_PIXEL;
const BUFFER_SIZE: usize = FRAME_SIZE * BYTES_PER_PIXEL;

pub struct FrameBuffer {
    buffer: [u8; BUFFER_SIZE],
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [0; BUFFER_SIZE],
        }
    }

    pub fn frame(&self) -> &[u8] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer = [0; BUFFER_SIZE];
    }

    pub fn draw(&mut self, sprite: &[u8], coordinates: (usize, usize)) -> bool {
        // wrap the starting coordinates
        let start_x = coordinates.0 % FRAME_WIDTH;
        let start_y = coordinates.1 % FRAME_HEIGHT;

        let mut has_collided = false;
        // take each row (byte) of the sprite
        for (i, byte) in sprite.iter().enumerate() {
            // iterate over each bit
            for j in 0..u8::BITS as usize {
                let x = start_x + j;
                let y = start_y + i;
                // stop drawing if we go off the screen
                if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                    continue;
                }
                // check the state of the bit
                let bit = (byte >> (u8::BITS as usize - 1 - j)) & 0x1;
                if bit == 1 {
                    let pixel_offset = (x * BYTES_PER_PIXEL) + y * BYTES_PER_ROW;
                    let pixel = self
                        .buffer
                        .get_mut(pixel_offset..pixel_offset + BYTES_PER_PIXEL)
                        .expect("pixel out of bounds");

                    if pixel == &PIXEL_ON {
                        pixel.copy_from_slice(&PIXEL_OFF);
                        has_collided = true;
                    } else {
                        pixel.copy_from_slice(&PIXEL_ON);
                    }
                }
            }
        }
        has_collided
    }
}
