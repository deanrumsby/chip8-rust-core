use std::env;
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use chip8_core_rs::{Chip8, Pixel, Key, KeyState, PIXELS_WIDTH, PIXELS_HEIGHT};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_logical_size(PIXELS_WIDTH as u32, PIXELS_HEIGHT as u32).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load(Path::new(&env::args().nth(1).unwrap()));

    if env::args().nth(2).is_some() {
        chip8.set_speed(env::args().nth(2).unwrap().parse::<u64>().unwrap());
    }

    let key_map = HashMap::from([
        (Keycode::Num1, Key::Key(0x1)),
        (Keycode::Num2, Key::Key(0x2)),
        (Keycode::Num3, Key::Key(0x3)),
        (Keycode::Num4, Key::Key(0xC)),
        (Keycode::Q, Key::Key(0x4)),
        (Keycode::W, Key::Key(0x5)),
        (Keycode::E, Key::Key(0x6)),
        (Keycode::R, Key::Key(0xD)),
        (Keycode::A, Key::Key(0x7)),
        (Keycode::S, Key::Key(0x8)),
        (Keycode::D, Key::Key(0x9)),
        (Keycode::F, Key::Key(0xE)),
        (Keycode::Z, Key::Key(0xA)),
        (Keycode::X, Key::Key(0x0)),
        (Keycode::C, Key::Key(0xB)),
        (Keycode::V, Key::Key(0xF)),
        ]);

        chip8.start();
        let mut frame_start = std::time::Instant::now();
        
        'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(code), .. } => chip8.handle_key_event(key_map[&code], KeyState::Pressed),
                Event::KeyUp { keycode: Some(code), .. } => chip8.handle_key_event(key_map[&code], KeyState::Released),
                _ => {},
            }
        }

        chip8.step();

        if frame_start.elapsed() > Duration::from_millis(1000 / 200) {
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

        chip8.tick();
    }
}