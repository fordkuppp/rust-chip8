use std::fs::File;
use std::io::Read;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::WindowBuilder;
use crate::chip8::Chip8;

mod chip8;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    println!("Hello, world!");
    // TODO: Set up render system and input
    let event_loop = EventLoop::new();
    let mut window = {
        let size = LogicalSize::new(WIDTH as u32, HEIGHT as u32);
        WindowBuilder::new()
            .with_title("Chip-8")
            // .with_inner_size(size)
            // .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    // Initialize chip8 and load rom into memory TODO: take path from argument, open file from chip8 instance instead
    let mut chip8 = Chip8::new();
    // let mut rom = File::open(&"roms/chip8-test-suite.ch8").expect("Unable to open file");
    // let mut rom = File::open(&"roms/IBM Logo.ch8").expect("Unable to open file");
    let mut rom = File::open(&"roms/test_opcode.ch8").expect("Unable to open file");

    let mut buf = Vec::new();
    rom.read_to_end(&mut buf).unwrap();
    chip8.load(&buf);

    event_loop.run(move |event, _, control_flow| {
        // Handle draw event
        if chip8.draw_flag {
            window.request_redraw();
        }
        match event {
            Event::WindowEvent {
                event:
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                },
                ..
            } => match key {
                VirtualKeyCode::F => {
                    println!("f is pressed!")
                }
                _ => (),
            }
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                control_flow.set_exit();
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let pixel_state = chip8.screen[i];
                    let rgba = if pixel_state {
                        [0x00, 0xFF, 0x00, 0xFF]
                    } else {
                        [0x00, 0x00, 0x00, 0xFF]
                    };
                    pixel.copy_from_slice(&rgba);
                }
                pixels.render().unwrap();
            },
            _ => ()
        }
        // Run next tick
        chip8.tick();
    });
}
