use chip8_core_rs::keys::{Key, KeyState};
use chip8_core_rs::Chip8;
use pixels::{Pixels, SurfaceTexture};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use winit::dpi::LogicalSize;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
// fn main() {
//     let mut chip8 = Chip8::new();
//     let binding = env::args().nth(1).unwrap();
//     let path = Path::new(binding.as_str());
//     chip8.load(path);
//     let mut count = 0;

//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_inner_size(LogicalSize::new(64 * 10, 32 * 10))
//         .build(&event_loop)
//         .unwrap();

//     let mut pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(64, 32, surface_texture).unwrap()
//     };

//     let key_map = HashMap::from([
//         (VirtualKeyCode::Key1, Key::Key(0x1)),
//         (VirtualKeyCode::Key2, Key::Key(0x2)),
//         (VirtualKeyCode::Key3, Key::Key(0x3)),
//         (VirtualKeyCode::Key4, Key::Key(0xc)),
//         (VirtualKeyCode::Q, Key::Key(0x4)),
//         (VirtualKeyCode::W, Key::Key(0x5)),
//         (VirtualKeyCode::E, Key::Key(0x6)),
//         (VirtualKeyCode::R, Key::Key(0xd)),
//         (VirtualKeyCode::A, Key::Key(0x7)),
//         (VirtualKeyCode::S, Key::Key(0x8)),
//         (VirtualKeyCode::D, Key::Key(0x9)),
//         (VirtualKeyCode::F, Key::Key(0xe)),
//         (VirtualKeyCode::Z, Key::Key(0xa)),
//         (VirtualKeyCode::X, Key::Key(0x0)),
//         (VirtualKeyCode::C, Key::Key(0xb)),
//         (VirtualKeyCode::V, Key::Key(0xf)),
//     ]);

//     chip8.clock.start();

//     event_loop.run(move |event, _, control_flow| {
//         control_flow.set_poll();

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 window_id,
//             } if window_id == window.id() => *control_flow = ControlFlow::Exit,

//             Event::MainEventsCleared => {
//                 chip8.step();

//                 if chip8.cpu.redraw == true {
//                     let buffer = chip8.get_frame_buffer();
//                     for (pixel, chunk) in buffer
//                         .iter()
//                         .zip(pixels.get_frame_mut().chunks_exact_mut(4))
//                     {
//                         let slice = match *pixel {
//                             Pixel::On => [255, 255, 255, 255],
//                             Pixel::Off => [0, 0, 0, 255],
//                         };
//                         chunk.copy_from_slice(&slice);
//                         window.request_redraw();
//                     }
//                     // if let Err(err) = pixels.render() {
//                     //     println!("pixels.render() failed: {}", err);
//                     //     *control_flow = ControlFlow::Exit;
//                     //     return;
//                     // }
//                 }

//                 // chip8.clock.tick();
//             }

//             Event::RedrawRequested(_) => {
//                 println!("REDRAW {}", count);
//                 count += 1;
//                 pixels.render().unwrap();
//             }

//             Event::WindowEvent {
//                 window_id: _,
//                 event:
//                     WindowEvent::KeyboardInput {
//                         device_id: _,
//                         input,
//                         is_synthetic: _,
//                     },
//             } => {
//                 if let VirtualKeyCode::Space = input.virtual_keycode.unwrap() {
//                     // chip8.step();
//                     // println!("{:?}", chip8.get_frame_buffer());
//                     // println!("{:?}", pixels);
//                     // chip8.cpu.redraw = true;
//                     window.request_redraw();
//                 } else {
//                     let key = key_map.get(&input.virtual_keycode.unwrap()).unwrap();
//                     match input.state {
//                         ElementState::Pressed => chip8.handle_key_event(*key, KeyState::Down),
//                         ElementState::Released => chip8.handle_key_event(*key, KeyState::Up),
//                     }
//                 }
//             }
//             _ => (),
//         }
//     });
// }
