const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COUNT: usize = WIDTH * HEIGHT;

#[derive(Clone, Copy)]
pub enum Pixel {
    On,
    Off,
}

const MSB_MASK: u8 = 0b1000_0000;

pub struct Frame {
    pixels: [Pixel; PIXEL_COUNT],
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pixels: [Pixel::Off; PIXEL_COUNT],
        }
    }

    pub fn get_pixel_buffer(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn clear(&mut self) {
        self.pixels = [Pixel::Off; PIXEL_COUNT];
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
        let pixels = &mut self.pixels[offset..offset + u8::BITS as usize];

        for (column_index, pixel) in pixels.iter_mut().enumerate() {
            if Self::is_offscreen((x + column_index, y)) {
                continue;
            }
            let should_toggle = (byte & MSB_MASK) != 0;
            if should_toggle {
                match *pixel {
                    Pixel::On => {
                        *pixel = Pixel::Off;
                        has_byte_collided = true;
                    }
                    Pixel::Off => *pixel = Pixel::On,
                }
            }
            byte <<= 1;
        }
        has_byte_collided
    }

    fn determine_true_coordinates(coordinates: (usize, usize)) -> (usize, usize) {
        let (x, y) = coordinates;
        (x % WIDTH, y % HEIGHT)
    }

    fn convert_coordinates_to_offset(coordinates: (usize, usize)) -> usize {
        let (x, y) = coordinates;
        x + y * WIDTH
    }

    fn is_offscreen(coordinates: (usize, usize)) -> bool {
        let (x, y) = coordinates;
        x >= (y + 1) * WIDTH
    }
}
