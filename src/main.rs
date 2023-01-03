use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::chip8::Chip8;

mod chip8;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        WindowBuilder::new()
            .with_title("Chip-8")
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
    let mut rom = File::open("roms/chip8-test-suite.ch8").expect("Unable to open file");
    // let mut rom = File::open("roms/test_opcode.ch8").expect("Unable to open file");

    let mut buf = Vec::new();
    rom.read_to_end(&mut buf).unwrap();
    chip8.load(&buf);

    // TODO: make timer actually work
    let timer_length = Duration::new(0, 16666666); // TODO: remove 1 sec
    event_loop.run(move |event, _, control_flow| {
        // Handle draw event
        if chip8.draw_flag {
            // Use set_wait_until to draw at 60 fps
            control_flow.set_wait_until(Instant::now() + timer_length);
            window.request_redraw();
        }
        match event {
            Event::WindowEvent {
                event:
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        scancode: key,
                        state,
                        ..
                    },
                    ..
                },
                ..
            } => match state {
                // 1,2,3,4   <= keyboard, chip8 =>  1,2,3,c
                // q,w,e,r                          4,5,6,d
                // a,s,d,f                          7,8,9,e
                // z,x,c,v                          a,0,b,f

                // Use keyboard scan codes set 1
                ElementState::Pressed => match key {
                    0x02 => { // 1 <=> 1
                        chip8.key[1] = true;
                    }
                    0x03 => { // 2 <=> 2
                        chip8.key[2] = true;
                    }
                    0x04 => { // 3 <=> 3
                        chip8.key[3] = true;
                    }
                    0x05 => { // 4 <=> c
                        chip8.key[0xc] = true;
                    }
                    0x10 => { // q <=> 4
                        chip8.key[4] = true;
                    }
                    0x11 => { // w <=> 5
                        chip8.key[5] = true;
                    }
                    0x12 => { // e <=> 6
                        chip8.key[6] = true;
                    }
                    0x13 => { // r <=> d
                        chip8.key[0xd] = true;
                    }
                    0x1e => { // a <=> 7
                        chip8.key[7] = true;
                    }
                    0x1f => { // s <=> 8
                        chip8.key[8] = true;
                    }
                    0x20 => { // d <=> 9
                        chip8.key[9] = true;
                    }
                    0x21 => { // f <=> e
                        chip8.key[0xe] = true;
                    }
                    0x2c => { // z <=> a
                        chip8.key[12] = true;
                    }
                    0x2d => { // x <=> 0
                        chip8.key[13] = true;
                    }
                    0x2e => { // c <=> b
                        chip8.key[14] = true;
                    }
                    0x2f => { // v <=> f
                        chip8.key[15] = true;
                    }
                    _ => (),
                }
                ElementState::Released => match key {
                    0x02 => { // 1 <=> 1
                        chip8.key[1] = false;
                    }
                    0x03 => { // 2 <=> 2
                        chip8.key[2] = false;
                    }
                    0x04 => { // 3 <=> 3
                        chip8.key[3] = false;
                    }
                    0x05 => { // 4 <=> c
                        chip8.key[0xc] = false;
                    }
                    0x10 => { // q <=> 4
                        chip8.key[4] = false;
                    }
                    0x11 => { // w <=> 5
                        chip8.key[5] = false;
                    }
                    0x12 => { // e <=> 6
                        chip8.key[6] = false;
                    }
                    0x13 => { // r <=> d
                        chip8.key[0xd] = false;
                    }
                    0x1e => { // a <=> 7
                        chip8.key[7] = false;
                    }
                    0x1f => { // s <=> 8
                        chip8.key[8] = false;
                    }
                    0x20 => { // d <=> 9
                        chip8.key[9] = false;
                    }
                    0x21 => { // f <=> e
                        chip8.key[0xe] = false;
                    }
                    0x2c => { // z <=> a
                        chip8.key[12] = false;
                    }
                    0x2d => { // x <=> 0
                        chip8.key[13] = false;
                    }
                    0x2e => { // c <=> b
                        chip8.key[14] = false;
                    }
                    0x2f => { // v <=> f
                        chip8.key[15] = false;
                    }
                    _ => (),
                }

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
