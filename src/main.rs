use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::audio::Audio;
use crate::chip8::Chip8;

mod chip8;
mod timer;
mod audio;

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
    let audio_stream = Audio::new();
    let mut chip8 = Chip8::new();
    let mut rom = File::open("roms/IBM Logo.ch8").expect("Unable to open file");
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        rom = File::open(&args[1]).expect("Unable to open file");
    }
    let mut rom_buf = Vec::new();
    rom.read_to_end(&mut rom_buf).unwrap();
    chip8.load(&rom_buf);

    let timer_length = Duration::new(0, 16666666); // This is 60Hz
    event_loop.run(move |event, _, control_flow| {
        if chip8.draw_flag {
            window.request_redraw();
        }

        if chip8.timer.get_st() != 0 {
            audio_stream.play();
        } else {
            audio_stream.pause();
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
                        chip8.key[0x1] = true;
                    }
                    0x03 => { // 2 <=> 2
                        chip8.key[0x2] = true;
                    }
                    0x04 => { // 3 <=> 3
                        chip8.key[0x3] = true;
                    }
                    0x05 => { // 4 <=> c
                        chip8.key[0xC] = true;
                    }
                    0x10 => { // q <=> 4
                        chip8.key[0x4] = true;
                    }
                    0x11 => { // w <=> 5
                        chip8.key[0x5] = true;
                    }
                    0x12 => { // e <=> 6
                        chip8.key[0x6] = true;
                    }
                    0x13 => { // r <=> d
                        chip8.key[0xD] = true;
                    }
                    0x1e => { // a <=> 7
                        chip8.key[0x7] = true;
                    }
                    0x1f => { // s <=> 8
                        chip8.key[0x8] = true;
                    }
                    0x20 => { // d <=> 9
                        chip8.key[0x9] = true;
                    }
                    0x21 => { // f <=> e
                        chip8.key[0xE] = true;
                    }
                    0x2c => { // z <=> a
                        chip8.key[0xA] = true;
                    }
                    0x2d => { // x <=> 0
                        chip8.key[0x0] = true;
                    }
                    0x2e => { // c <=> b
                        chip8.key[0xB] = true;
                    }
                    0x2f => { // v <=> f
                        chip8.key[0xF] = true;
                    }
                    _ => (),
                }
                ElementState::Released => match key {
                    0x02 => { // 1 <=> 1
                        chip8.key[0x1] = false;
                    }
                    0x03 => { // 2 <=> 2
                        chip8.key[0x2] = false;
                    }
                    0x04 => { // 3 <=> 3
                        chip8.key[0x3] = false;
                    }
                    0x05 => { // 4 <=> c
                        chip8.key[0xC] = false;
                    }
                    0x10 => { // q <=> 4
                        chip8.key[0x4] = false;
                    }
                    0x11 => { // w <=> 5
                        chip8.key[0x5] = false;
                    }
                    0x12 => { // e <=> 6
                        chip8.key[0x6] = false;
                    }
                    0x13 => { // r <=> d
                        chip8.key[0xD] = false;
                    }
                    0x1e => { // a <=> 7
                        chip8.key[0x7] = false;
                    }
                    0x1f => { // s <=> 8
                        chip8.key[0x8] = false;
                    }
                    0x20 => { // d <=> 9
                        chip8.key[0x9] = false;
                    }
                    0x21 => { // f <=> e
                        chip8.key[0xE] = false;
                    }
                    0x2c => { // z <=> a
                        chip8.key[0xA] = false;
                    }
                    0x2d => { // x <=> 0
                        chip8.key[0x0] = false;
                    }
                    0x2e => { // c <=> b
                        chip8.key[0xB] = false;
                    }
                    0x2f => { // v <=> f
                        chip8.key[0xF] = false;
                    }
                    _ => (),
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                control_flow.set_exit();
            }
            Event::RedrawRequested(_) => {
                // Use set_wait_until to draw at 60 fps
                control_flow.set_wait_until(Instant::now() + timer_length);

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
            }
            _ => ()
        }
        chip8.tick();
    });
}
