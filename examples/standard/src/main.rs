use chip8_core::Chip8;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::path::Path;
use std::fs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut chip8 = Chip8::new();
    let rom = fs::read(Path::new(&env::args().nth(1).unwrap())).unwrap();

    chip8.load(rom.as_slice());
    chip8.start(current_time());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            
            Event::MainEventsCleared => {
                chip8.emulate(current_time());
            },
            _ => (),
        }
    });
}

fn current_time() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("Current time: {:?}", since_the_epoch);

    since_the_epoch.as_micros() as u64
}
