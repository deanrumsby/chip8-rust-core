pub const PIXELS_WIDTH: usize = 64;
pub const PIXELS_HEIGHT: usize = 32;

#[derive(Clone, Copy)]
pub enum Pixel {
    On,
    Off,
}

pub struct PixelBuffer {
    pixels: [Pixel; PIXELS_WIDTH * PIXELS_HEIGHT],
}

impl PixelBuffer {
    pub fn new() -> Self {
        PixelBuffer {
            pixels: [Pixel::Off; PIXELS_WIDTH * PIXELS_HEIGHT],
        }
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn clear(&mut self) {
        self.pixels = [Pixel::Off; PIXELS_WIDTH * PIXELS_HEIGHT];
    }

    pub fn draw(&mut self, sprite: &[u8], coordinates: (usize, usize)) -> bool {
        let start_x = coordinates.0 % PIXELS_WIDTH;
        let start_y = coordinates.1 % PIXELS_HEIGHT;

        let mut has_collided = false;
        for (i, byte) in sprite.iter().enumerate() {
            for j in 0..u8::BITS as usize {
                let x = start_x + j;
                let y = start_y + i;
                if x >= PIXELS_WIDTH || y >= PIXELS_HEIGHT {
                    continue;
                }
                let offset = x + y * PIXELS_WIDTH;
                let bit = (byte >> (u8::BITS as usize - 1 - j)) & 0x1;
                let pixel = self.pixels[offset];
                if bit == 1 {
                    match pixel {
                        Pixel::On => {
                            self.pixels[offset] = Pixel::Off;
                            has_collided = true;
                        }
                        Pixel::Off => {
                            self.pixels[offset] = Pixel::On;
                        }
                    }
                }
            }
        }
        has_collided
    }
}
