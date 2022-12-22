// Reference: https://aquova.net/chip8/chip8.pdf, https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

use sdl2::pixels::Color;
use crate::display::Display;
use crate::input::Input;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub v_register: [u8; 16],
    pub i_register: u16,
    pub program_counter: u16,
    pub display: [bool; WIDTH*HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub stack_pointer: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut new_chip8 = Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v_register: [0; 16],
            i_register: 0,
            program_counter: 0x200,         // start at 0x200 per original chip-8
            display: [false; WIDTH*HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
        new_chip8.memory[0x050..=0x09F].copy_from_slice(&FONTSET);
        new_chip8
    }

    // Reset everything to original state
    pub fn reset(&mut self) {
        self.opcode = 0;
        self.memory = [0; 4096];
        self.v_register = [0; 16];
        self.i_register = 0;
        self.program_counter = 0x200;        // start at 0x200 per original chip-8
        self.display = [false; WIDTH*HEIGHT];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.memory[0x050..0x09F].copy_from_slice(&FONTSET);
    }

    // Load data into memory TODO: Take in path instead
    pub fn load(&mut self, data: &[u8]) {
        self.memory[0x200..(0x200 + data.len())].copy_from_slice(data);
    }

    // Emulate one cycle
    pub fn tick(&mut self) {
        // Fetch
        let opcode = self.fetch();
        // Decode
        let nibbles = self.decode();
        // Execute
        self.execute(nibbles);
    }

    // Chip-8 opcode is 2 bytes long, so merge 2 bytes from memory and increment program counter by 2
    fn fetch(&mut self) -> u16{
        self.opcode = (self.memory[self.pc]) << 8 | (self.memory[(self.pc) + 1]);
        self.program_counter += 2;
        opcode
    }

    // Decode into tuple of 4 nibbles
    fn decode(&mut self) -> (u16, u16, u16, u16) {
        (
            (self.opcode & 0xF000) >> 12,
            (self.opcode & 0x0F00) >> 8,
            (self.opcode & 0x00F0) >> 4,
            (self.opcode & 0x000F)
        )
    }

    fn push(&mut self, val: u16) {
        self.stack[self.stack_pt] = val;
        self.stack_pt += 1;
    }

    fn pop(&mut self) {
        self.stack_pt -= 1;
        self.stack[self.stack_pt];
    }

    // Execute opcode
    fn execute(&mut self, nibbles: (u16, u16, u16, u16)) {
        let nnn = (self.opcode & 0x0FFF);
        let nn = (self.opcode & 0x00FF) as u8;
        match nibbles {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.op_00e0(),
            (1, _, _, _) => self.op_1nnn(nnn),
            (6, _, _, _) => self.op_6xnn(x, nn),
            (7, _, _, _) => self.op_7xnn(x, nn),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (_, _, _, _) => unimplemented!("Unimplemented")
        }
    }

    // Clear display
    fn op_00e0(&mut self) {
        self.display.clear()
    }

    // Jump to address NNN
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn
    }

    // Set register VX to NN
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.register[x] = nn
    }

    // Add NN to register VX
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        self.register[x] += nn
    }

    // Set index register to NNN
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn
    }

    // Draw
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let x_coord = self.register[x];
        let y_coord = self.register[y];
        let mut flipped = false;

        let mut points = Vec::new();
        for y_line in 0..n {
            let y = (self.register[y] + y_line as u8) as usize % 32;
            let data = self.memory[(self.i + y_line as u16) as usize];
            for x_line in 0..8 {
                let x = (self.register[x] + x_line) as usize % 64;
                println!("{} {}", x,y);
                let idx = (x + 32 * y) as usize;
                flipped |= self.framebuffer[idx];
                self.framebuffer[idx] ^= true;
                println!("{:?}",&idx);
                points.push([x,y]);
            }
        }
        println!("{:?}", &self.framebuffer);
        if flipped {
            self.display.draw(&points, Color::RGB(255, 0, 0));
            self.register[0xF] = 1;
        } else {
            self.display.draw(&points, Color::RGB(0, 255, 0));
            self.register[0xF] = 0;
        }

    }
}