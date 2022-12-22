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
    // TODO: Set up render system and input
    let mut display = Display::new();
    // Initialize chip8 and load rom into memory TODO: take path from argument, open file from chip8 instance instead
    let mut chip8 = Chip8::new();
    let mut rom = File::open(&"roms/IBM Logo.ch8").expect("Unable to open file");
    let mut buf = Vec::new();
    rom.read_to_end(&mut buf).unwrap();
    chip8.load(&buf);

    loop {
        // Emulate one cycle
        chip8.tick();
        if chip8.draw_flag {
            display.draw(chip8.screen)
        }

        // TODO: If draw flag is set then update display
        // let sprite = vec![
        //     [14,13],
        //     [0,1],
        //     [3,4]
        // ];
        // display.draw(&sprite);
    }
}
