use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use chip8_core::{Chip8, Key, KeyState, FRAME_HEIGHT, FRAME_WIDTH, PIXEL_ON};

const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / 60);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas
        .set_logical_size(FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
        .unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    let rom = fs::read(Path::new(&env::args().nth(1).unwrap())).unwrap();
    chip8.load(rom.as_slice());

    if env::args().nth(2).is_some() {
        chip8.set_speed(env::args().nth(2).unwrap().parse::<u64>().unwrap());
    }

    let key_map = HashMap::from([
        (Keycode::Num1, Key::Key1),
        (Keycode::Num2, Key::Key2),
        (Keycode::Num3, Key::Key3),
        (Keycode::Num4, Key::KeyC),
        (Keycode::Q, Key::Key4),
        (Keycode::W, Key::Key5),
        (Keycode::E, Key::Key6),
        (Keycode::R, Key::KeyD),
        (Keycode::A, Key::Key7),
        (Keycode::S, Key::Key8),
        (Keycode::D, Key::Key9),
        (Keycode::F, Key::KeyE),
        (Keycode::Z, Key::KeyA),
        (Keycode::X, Key::Key0),
        (Keycode::C, Key::KeyB),
        (Keycode::V, Key::KeyF),
    ]);

    'running: loop {
        let frame_start = Instant::now();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => match key_map.get(&code) {
                    Some(&key) => chip8.handle_key_event(key, KeyState::Pressed),
                    _ => {}
                }
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match key_map.get(&code) {
                    Some(&key) => chip8.handle_key_event(key, KeyState::Released),
                    _ => {}
                }
                _ => {}
            }
        }

        chip8.emulate_frame(1);
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let pixels = chip8.frame();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (offset, pixel_state) in pixels.chunks(4).enumerate() {
            let pixel = pixel_state[3];
            let (x, y) = (offset % FRAME_WIDTH, offset / FRAME_WIDTH);
            match pixel {
                PIXEL_ON => canvas
                    .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                    .unwrap(),
                _ => {}
            }
        }
        canvas.present();
       
        let dt = frame_start.elapsed();
        if dt < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - dt);
        }
    }
}
