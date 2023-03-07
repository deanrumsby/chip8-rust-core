use std::env;
use std::path::Path;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use chip8_core_rs::Chip8;
use chip8_core_rs::cpu::{Pixel, PIXELS_WIDTH, PIXELS_HEIGHT};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_logical_size(PIXELS_WIDTH as u32, PIXELS_HEIGHT as u32).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load(Path::new(&env::args().nth(1).unwrap()));
    
    chip8.clock.start();
    let mut frame_start = std::time::Instant::now();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        chip8.step();

        if frame_start.elapsed() > Duration::from_millis(1000 / 150) {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
    
            let pixels = chip8.pixels();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            
            for (offset, pixel) in pixels.iter().enumerate() {
                let (x, y) = (offset % PIXELS_WIDTH, offset / PIXELS_WIDTH);
                match pixel {
                    Pixel::On => canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap(),
                    _ => {},
                }
            }
            canvas.present();
            frame_start = std::time::Instant::now();
        }
        // The rest of the game loop goes here...

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        chip8.clock.tick();
    }
}