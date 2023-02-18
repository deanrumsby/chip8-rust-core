const WIDTH_PIXELS: usize = 64;
const HEIGHT_PIXELS: usize = 32;

const PIXEL_ON: u8 = u8::MAX;
const PIXEL_OFF: u8 = u8::MIN;

const MSB_MASK: u8 = 0b1000_0000;

pub struct Frame {
    buffer: [u8; 64 * 32],
}

impl Frame {
    pub fn new() -> Self {
        Self {
            buffer: [PIXEL_OFF; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [PIXEL_OFF; 64 * 32];
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], coordinates: (usize, usize)) -> bool {
        let (x, y) = Self::determine_true_coordinates(coordinates);
        let mut has_sprite_collided = false;
        for (row_index, byte) in sprite.iter().enumerate() {
            let has_byte_collided = self.draw_byte(*byte, (x, y + row_index));
            if has_byte_collided {
                has_sprite_collided = true;
            }
        }
        has_sprite_collided
    }

    fn draw_byte(&mut self, mut byte: u8, coordinates: (usize, usize)) -> bool {
        let (x, y) = coordinates;
        let mut has_byte_collided = false;
        let offset = Self::convert_coordinates_to_offset(coordinates);
        let pixels = &mut self.buffer[offset..offset + u8::BITS as usize];

        for (column_index, pixel) in pixels.iter_mut().enumerate() {
            if Self::is_offscreen((x + column_index, y)) {
                continue;
            }
            let should_toggle = (byte & MSB_MASK) != 0;
            if should_toggle {
                if *pixel == PIXEL_ON {
                    *pixel = PIXEL_OFF;
                    has_byte_collided = true;
                }
                *pixel = PIXEL_ON;
            }
            byte <<= 1;
        }
        has_byte_collided
    }

    fn determine_true_coordinates(coordinates: (usize, usize)) -> (usize, usize) {
        let (x, y) = coordinates;
        (x % WIDTH_PIXELS, y % HEIGHT_PIXELS)
    }

    fn convert_coordinates_to_offset(coordinates: (usize, usize)) -> usize {
        let (x, y) = coordinates;
        x + y * WIDTH_PIXELS
    }

    fn is_offscreen(coordinates: (usize, usize)) -> bool {
        let (x, y) = coordinates;
        x >= (y + 1) * WIDTH_PIXELS
    }
}
