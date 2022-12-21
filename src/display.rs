extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("chip8 demo", 1200, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_logical_size(WIDTH as u32, HEIGHT as u32).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        Display{canvas}
    }

    // Example input vec![ [14,13],[0,1],[3,4] ];
    pub fn draw(&mut self, sprite: &[[u32; 2]]) {
        let points: Vec<Point> = sprite.iter()
            .map(|p| Point::new(p[0] as i32, p[1] as i32))
            .collect::<Vec<Point>>();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_points(points.as_slice()).unwrap();
        self.canvas.present();
    }
}
// pub fn show() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();
//
//     let window = video_subsystem.window("rust-sdl2 demo", 1200, 600)
//         .position_centered()
//         .build()
//         .unwrap();
//
//     let mut canvas = window.into_canvas().build().unwrap();
//     canvas.set_logical_size(64, 32).unwrap();
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     let mut i = 0;
//     let mut rng = rand::thread_rng();
//
//     canvas.present();
//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. } |
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                     break 'running;
//                 }
//                 _ => {}
//             }
//         }
//         //     // The rest of the game loop goes here...
//         //
//         //     i = (i + 1) % 255;
//         canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//         //     let (w, h) = canvas.output_size().unwrap();
//         //     let mut points = [Point::new(0, 0); 256];
//         //     points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
//         //     // For performance, it's probably better to draw a whole bunch of points at once
//         //     canvas.draw_points(points.as_slice()).unwrap();
//         canvas.draw_point(Point::new(15, 15)).unwrap();
//         canvas.present();
//         // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }