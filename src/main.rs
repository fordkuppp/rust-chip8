use std::fs::File;
use std::io::Read;
use crate::chip8::Chip8;
use crate::display::Display;
use crate::input::Input;

mod chip8;
mod display;
mod input;

fn main() {
    println!("Hello, world!");
    let mut chip8 = Chip8::new();

    // TODO: take path from argument
    let mut rom = File::open(&"roms/IBM Logo.ch8").expect("Unable to open file");
    let mut buf = Vec::new();
    rom.read_to_end(&mut buf).unwrap();
    chip8.load(&buf);

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    loop {
        let key_result = input.poll();
        if key_result.is_err() {
            println!("Quitting");
            break
        }
        if key_result.unwrap() == 0x0 {
            println!("good!");
        }
        let sprite = vec![
            [14,13],
            [0,1],
            [3,4]
        ];
        display.draw(&sprite);
    }
}
