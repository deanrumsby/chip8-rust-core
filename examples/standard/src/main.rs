use chip8_core::{Chip8, Key, KeyState, FRAME_HEIGHT, FRAME_WIDTH};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{
    env, fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Chip8")
        .with_inner_size(winit::dpi::LogicalSize::new(640, 320))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(FRAME_WIDTH as u32, FRAME_HEIGHT as u32, surface_texture)?
    };

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

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == window.id() => {
                let key_code = match input.virtual_keycode {
                    Some(key) => match key {
                        VirtualKeyCode::Key1 => Some(Key::Key1),
                        VirtualKeyCode::Key2 => Some(Key::Key2),
                        VirtualKeyCode::Key3 => Some(Key::Key3),
                        VirtualKeyCode::Key4 => Some(Key::KeyC),
                        VirtualKeyCode::Q => Some(Key::Key4),
                        VirtualKeyCode::W => Some(Key::Key5),
                        VirtualKeyCode::E => Some(Key::Key6),
                        VirtualKeyCode::R => Some(Key::KeyD),
                        VirtualKeyCode::A => Some(Key::Key7),
                        VirtualKeyCode::S => Some(Key::Key8),
                        VirtualKeyCode::D => Some(Key::Key9),
                        VirtualKeyCode::F => Some(Key::KeyE),
                        VirtualKeyCode::Z => Some(Key::KeyA),
                        VirtualKeyCode::X => Some(Key::Key0),
                        VirtualKeyCode::C => Some(Key::KeyB),
                        VirtualKeyCode::V => Some(Key::KeyF),
                        _ => None,
                    },
                    None => None,
                };
                let key_state = match input.state {
                    ElementState::Pressed => KeyState::Pressed,
                    ElementState::Released => KeyState::Released,
                };

                match key_code {
                    Some(key) => chip8.handle_key_event(key, key_state),
                    None => (),
                }
            }

            Event::MainEventsCleared => {
                chip8.emulate(current_time());
                pixels.frame_mut().copy_from_slice(chip8.frame_buffer());
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

fn current_time() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_micros() as u64
}
