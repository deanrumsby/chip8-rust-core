use chip8_rust_core::frame::Pixel;
use chip8_rust_core::keys::{Key, KeyState};
use chip8_rust_core::Chip8;
use pixels::{Pixels, SurfaceTexture};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
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

    let key_map = HashMap::from([
        (VirtualKeyCode::Key1, Key::Key(0x1)),
        (VirtualKeyCode::Key2, Key::Key(0x2)),
        (VirtualKeyCode::Key3, Key::Key(0x3)),
        (VirtualKeyCode::Key4, Key::Key(0xc)),
        (VirtualKeyCode::Q, Key::Key(0x4)),
        (VirtualKeyCode::W, Key::Key(0x5)),
        (VirtualKeyCode::E, Key::Key(0x6)),
        (VirtualKeyCode::R, Key::Key(0xd)),
        (VirtualKeyCode::A, Key::Key(0x7)),
        (VirtualKeyCode::S, Key::Key(0x8)),
        (VirtualKeyCode::D, Key::Key(0x9)),
        (VirtualKeyCode::F, Key::Key(0xe)),
        (VirtualKeyCode::Z, Key::Key(0xa)),
        (VirtualKeyCode::X, Key::Key(0x0)),
        (VirtualKeyCode::C, Key::Key(0xb)),
        (VirtualKeyCode::V, Key::Key(0xf)),
    ]);

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
                    let slice = match *pixel {
                        Pixel::On => [255, 255, 255, 255],
                        Pixel::Off => [0, 0, 0, 255],
                    };
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
            } => {
                let key = key_map.get(&input.virtual_keycode.unwrap()).unwrap();
                match input.state {
                    ElementState::Pressed => chip8.handle_key_event(*key, KeyState::Down),
                    ElementState::Released => chip8.handle_key_event(*key, KeyState::Up),
                }
            }
            _ => (),
        }

        chip8.step();
        // chip8.clock.tick();
        window.request_redraw();
    });
}
