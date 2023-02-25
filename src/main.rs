use chip8_rust_core::keys::Key;
use chip8_rust_core::Chip8;
use pixels::{Pixels, SurfaceTexture};
use std::env;
use std::path::Path;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let mut chip8 = Chip8::new();
    let binding = env::args().nth(1).unwrap();
    let path = Path::new(binding.as_str());
    chip8.load(path);
    // chip8.clock.start();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(64, 32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(_) => {
                let buffer = chip8.get_frame_buffer();
                for (pixel, chunk) in buffer
                    .iter()
                    .zip(pixels.get_frame_mut().chunks_exact_mut(4))
                {
                    let slice = [*pixel, *pixel, *pixel, u8::MAX];
                    chunk.copy_from_slice(&slice);
                }
                if let Err(err) = pixels.render() {
                    println!("pixels.render() failed: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            Event::WindowEvent {
                window_id: _,
                event:
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    },
            } => match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Key1 => chip8.handle_key_press(Key::Key(0x1)),
                VirtualKeyCode::Key2 => chip8.handle_key_press(Key::Key(0x2)),
                VirtualKeyCode::Key3 => chip8.handle_key_press(Key::Key(0x3)),
                VirtualKeyCode::Key4 => chip8.handle_key_press(Key::Key(0xc)),
                VirtualKeyCode::Q => chip8.handle_key_press(Key::Key(0x4)),
                VirtualKeyCode::W => chip8.handle_key_press(Key::Key(0x5)),
                VirtualKeyCode::E => chip8.handle_key_press(Key::Key(0x6)),
                VirtualKeyCode::R => chip8.handle_key_press(Key::Key(0xd)),
                VirtualKeyCode::A => chip8.handle_key_press(Key::Key(0x7)),
                VirtualKeyCode::S => chip8.handle_key_press(Key::Key(0x8)),
                VirtualKeyCode::D => chip8.handle_key_press(Key::Key(0x9)),
                VirtualKeyCode::F => chip8.handle_key_press(Key::Key(0xe)),
                VirtualKeyCode::Z => chip8.handle_key_press(Key::Key(0xa)),
                VirtualKeyCode::X => chip8.handle_key_press(Key::Key(0x0)),
                VirtualKeyCode::C => chip8.handle_key_press(Key::Key(0xb)),
                VirtualKeyCode::V => chip8.handle_key_press(Key::Key(0xf)),
                _ => {}
            },
            _ => (),
        }

        chip8.step();
        // chip8.clock.tick();
        window.request_redraw();
    });
}
