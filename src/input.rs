use sdl2::{EventPump, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Input {
    event_pump: EventPump,
}

impl Input {
    pub fn new(sdl_context: &Sdl) -> Input {
        Input {event_pump: sdl_context.event_pump().unwrap()}
    }

    // Err(0) means no input, Err(1) means quit program
    pub fn poll(&mut self) -> Result<u16, u8> {
        // for event in self.event_pump.poll_iter() {
        if let Some(event) = self.event_pump.poll_iter().next() {
            let key: Result<u16, u8> = match event {
                Event::Quit {..} | Event::KeyDown {scancode: Some(Scancode::Escape), ..} => Err(1),
                Event::KeyDown {scancode: Some(Scancode::Num1), ..} => Ok(0x0),
                Event::KeyDown {scancode: Some(Scancode::Num2), ..} => Ok(0x1),
                Event::KeyDown {scancode: Some(Scancode::Num3), ..} => Ok(0x2),
                Event::KeyDown {scancode: Some(Scancode::Num4), ..} => Ok(0x3),
                Event::KeyDown {scancode: Some(Scancode::Q), ..} => Ok(0x4),
                Event::KeyDown {scancode: Some(Scancode::W), ..} => Ok(0x5),
                Event::KeyDown {scancode: Some(Scancode::E), ..} => Ok(0x6),
                Event::KeyDown {scancode: Some(Scancode::R), ..} => Ok(0x7),
                Event::KeyDown {scancode: Some(Scancode::A), ..} => Ok(0x8),
                Event::KeyDown {scancode: Some(Scancode::S), ..} => Ok(0x9),
                Event::KeyDown {scancode: Some(Scancode::D), ..} => Ok(0xa),
                Event::KeyDown {scancode: Some(Scancode::F), ..} => Ok(0xb),
                Event::KeyDown {scancode: Some(Scancode::Z), ..} => Ok(0xc),
                Event::KeyDown {scancode: Some(Scancode::X), ..} => Ok(0xd),
                Event::KeyDown {scancode: Some(Scancode::C), ..} => Ok(0xe),
                Event::KeyDown {scancode: Some(Scancode::V), ..} => Ok(0xf),
                _ => Ok(0xFF)
            };
            return key
        }
        Ok(0xFF)
    }
}