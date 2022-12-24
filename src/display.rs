extern crate sdl2;

use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};


const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pixels: Pixels,
}

impl Display {
    pub fn new(event_loop: EventLoop<()>) -> Display {
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

        pixels.render().unwrap();
        Display{pixels}
    }

    pub fn draw(&mut self, screen: [bool; 2048]) {
        let frame = self.pixels.get_frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_state = screen[i];
            let rgba = if pixel_state {
                [0x00, 0xFF, 0x00, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };
            pixel.copy_from_slice(&rgba);
        }
        self.pixels.render().unwrap();
    }
}