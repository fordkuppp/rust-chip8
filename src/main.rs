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
        // Set key to be empty (not pressing anything)
        chip8.key = [false; 16];

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
                // Keys are     1,2,3,4     corresponding to 16 boolean in chip8.key
                //              q,w,e,r
                //              a,s,d,f
                //              z,x,c,v
                VirtualKeyCode::Key1 => {
                    chip8.key[0] = true;
                }
                VirtualKeyCode::Key2 => {
                    chip8.key[1] = true;
                }
                VirtualKeyCode::Key3 => {
                    chip8.key[2] = true;
                }
                VirtualKeyCode::Key4 => {
                    chip8.key[3] = true;
                }
                VirtualKeyCode::Q => {
                    chip8.key[4] = true;
                }
                VirtualKeyCode::W => {
                    chip8.key[5] = true;
                }
                VirtualKeyCode::E => {
                    chip8.key[6] = true;
                }
                VirtualKeyCode::R => {
                    chip8.key[7] = true;
                }
                VirtualKeyCode::A => {
                    chip8.key[8] = true;
                }
                VirtualKeyCode::S => {
                    chip8.key[9] = true;
                }
                VirtualKeyCode::D => {
                    chip8.key[10] = true;
                }
                VirtualKeyCode::F => {
                    chip8.key[11] = true;
                }
                VirtualKeyCode::Z => {
                    chip8.key[12] = true;
                }
                VirtualKeyCode::X => {
                    chip8.key[13] = true;
                }
                VirtualKeyCode::C => {
                    chip8.key[14] = true;
                }
                VirtualKeyCode::V => {
                    chip8.key[15] = true;
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
