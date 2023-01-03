// Reference: https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/, https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

use rand::Rng;
use winit::event::Event;

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
    pub pc: u16,
    pub screen: [bool; WIDTH*HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub stack_ptr: u16,
    pub key: [bool; 16],
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_chip8 = Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v_register: [0; 16],
            i_register: 0,
            pc: 0x200,         // start at 0x200 per original chip-8
            screen: [false; WIDTH*HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_ptr: 0,
            key: [false; 16],
            draw_flag: false,
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
        self.pc = 0x200;        // start at 0x200 per original chip-8
        self.screen = [false; WIDTH*HEIGHT];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack = [0; 16];
        self.stack_ptr = 0;
        self.key = [false; 16];
        self.draw_flag = false;
        self.memory[0x050..0x09F].copy_from_slice(&FONTSET);
    }

    // Load data into memory TODO: Take in path instead
    pub fn load(&mut self, data: &[u8]) {
        self.memory[0x200..(0x200 + data.len())].copy_from_slice(data);
    }

    // Emulate one cycle
    pub fn tick(&mut self) {
        self.draw_flag = false;
        // Fetch
        self.fetch();
        // Decode
        let nibbles = self.decode();
        // Execute
        self.execute(nibbles);
    }

    // Chip-8 opcode is 2 bytes long, so merge 2 bytes from memory and increment program counter by 2
    fn fetch(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;
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

    // Execute opcode
    fn execute(&mut self, nibbles: (u16, u16, u16, u16)) {
        let nnn = (self.opcode & 0x0FFF);
        let nn = (self.opcode & 0x00FF);
        match nibbles {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.op_00e0(),
            (0, 0, 0xE, 0xE) => self.op_00ee(),
            (1, _, _, _) => self.op_1nnn(nnn),
            (2, _, _, _) => self.op_2nnn(nnn),
            (3, _, _, _) => self.op_3xnn(nibbles.1, nn),
            (4, _, _, _) => self.op_4xnn(nibbles.1, nn),
            (5, _, _, 0) => self.op_5xy0(nibbles.1, nibbles.2),
            (6, _, _, _) => self.op_6xnn(nibbles.1, nn),
            (7, _, _, _) => self.op_7xnn(nibbles.1, nn),
            (8, _, _, 0) => self.op_8xy0(nibbles.1, nibbles.2),
            (8, _, _, 1) => self.op_8xy1(nibbles.1, nibbles.2),
            (8, _, _, 2) => self.op_8xy2(nibbles.1, nibbles.2),
            (8, _, _, 3) => self.op_8xy3(nibbles.1, nibbles.2),
            (8, _, _, 4) => self.op_8xy4(nibbles.1, nibbles.2),
            (8, _, _, 5) => self.op_8xy5(nibbles.1, nibbles.2),
            (8, _, _, 6) => self.op_8xy6(nibbles.1, nibbles.2),
            (8, _, _, 7) => self.op_8xy7(nibbles.1, nibbles.2),
            (8, _, _, 0xE) => self.op_8xye(nibbles.1, nibbles.2),
            (9, _, _, 0) => self.op_9xy0(nibbles.1, nibbles.2),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nibbles.1, nnn),
            (0xC, _, _, _) => self.op_cxnn(nibbles.1, nn),
            (0xD, _, _, _) => self.op_dxyn(nibbles.1, nibbles.2, nibbles.3),
            (0xE, _, 9, 0xE) => self.op_ex9e(nibbles.1),
            (0xE, _, 0xA, 1) => self.op_exa1(nibbles.1),
            (0xF, _, 0, 7) => self.op_fx07(nibbles.1),
            (0xF, _, 0, 0xA) => self.op_fx0a(nibbles.1),
            (0xF, _, 1, 5) => self.op_fx15(nibbles.1),
            (0xF, _, 1, 8) => self.op_fx18(nibbles.1),
            (0xF, _, 1, 0xE) => self.op_fx1e(nibbles.1),
            (0xF, _, 2, 9) => self.op_fx29(nibbles.1),
            (0xF, _, 3, 3) => self.op_fx33(nibbles.1),
            (0xF, _, 5, 5) => self.op_fx55(nibbles.1),
            (0xF, _, 6, 5) => self.op_fx65(nibbles.1),
            (_, _, _, _) => () // Exhausted all possible opcodes
        }
    }

    // Clear display
    fn op_00e0(&mut self) {
        self.screen = [false; WIDTH*HEIGHT];
        self.draw_flag = true;
    }

    // Return from a subroutine
    fn op_00ee(&mut self) {
        self.stack_ptr -= 1;
        self.pc = self.stack[self.stack_ptr as usize];
    }

    // Jump to address NNN
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // Call subroutine at NNN
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack[self.stack_ptr as usize] = self.pc;
        self.stack_ptr += 1;
        self.pc = nnn;
    }

    // Skip next instruction if VX == NN
    fn op_3xnn(&mut self, x: u16, nn: u16) {
        if self.v_register[x as usize] == nn as u8 {
            self.pc += 2;
        }
    }

    // Skip next instruction if VX != NN
    fn op_4xnn(&mut self, x: u16, nn: u16) {
        if self.v_register[x as usize] != nn as u8 {
            self.pc += 2;
        }
    }

    // Skip next instruction if VX == VY
    fn op_5xy0(&mut self, x: u16, y: u16) {
        if self.v_register[x as usize] == self.v_register[y as usize] {
            self.pc += 2;
        }
    }

    // Set register VX to NN
    fn op_6xnn(&mut self, x: u16, nn: u16) {
        self.v_register[x as usize] = nn as u8;
    }

    // Add NN to register VX
    fn op_7xnn(&mut self, x: u16, nn: u16) {
        self.v_register[x as usize] = self.v_register[x as usize].overflowing_add(nn as u8).0;
    }

    // Set VX = VY
    fn op_8xy0(&mut self, x: u16, y: u16) {
        self.v_register[x as usize] = self.v_register[y as usize];
    }

    // Set VX to VX or VY
    fn op_8xy1(&mut self, x: u16, y: u16) {
        self.v_register[x as usize] |= self.v_register[y as usize];
    }

    // Set VX to VX and VY
    fn op_8xy2(&mut self, x: u16, y: u16) {
        self.v_register[x as usize] &= self.v_register[y as usize];
    }

    // Set VX to VX xor VY
    fn op_8xy3(&mut self, x: u16, y: u16) {
        self.v_register[x as usize] ^= self.v_register[y as usize];
    }

    // Add VY to VX. Set VF to 1 if there's carry
    fn op_8xy4(&mut self, x: u16, y: u16) {
        let (result, overflow) = self.v_register[x as usize].overflowing_add(self.v_register[y as usize]);
        self.v_register[x as usize] = result;
        self.v_register[0xf] = if overflow {1} else {0};
    }

    // Subtract VX with VY. If VX > VY then set VF to 1
    fn op_8xy5(&mut self, x: u16, y: u16) {
        let (result, overflow) = self.v_register[x as usize].overflowing_sub(self.v_register[y as usize]);
        self.v_register[x as usize] = result;
        self.v_register[0xf] = if overflow {0} else {1};
    }

    // Store LSB of VX to VF then right shift VF by 1
    //TODO This instruction is ambiguous, and only works on "modern' programs
    fn op_8xy6(&mut self, x: u16, y: u16) {
        self.v_register[0xf] = self.v_register[x as usize] & 0b00000001;
        self.v_register[x as usize] >>= 1;
    }

    // Subtract VY with VX. If VX < VY then set VF to 1
    fn op_8xy7(&mut self, x: u16, y: u16) {
        let (result, overflow) = self.v_register[y as usize].overflowing_sub(self.v_register[x as usize]);
        self.v_register[x as usize] = result;
        self.v_register[0xf] = if overflow {0} else {1};
    }
    // Store MSB of VX in VF then left shift VX by 1
    //TODO This instruction is ambiguous, and only works on "modern' programs
    fn op_8xye(&mut self, x: u16, y: u16) {
        self.v_register[0xf] = self.v_register[x as usize] >> 7;
        self.v_register[x as usize] <<= 1;
    }

    // Skip next instruction if VX != VY
    fn op_9xy0(&mut self, x: u16, y: u16) {
        if self.v_register[x as usize] != self.v_register[y as usize] {
            self.pc += 2;
        }
    }

    // Set index register to NNN
    fn op_annn(&mut self, nnn: u16) {
        self.i_register = nnn;
    }

    // Jump to nnn, plus value in VX
    //TODO This instruction is ambiguous, and only works on "modern' programs
    fn op_bnnn(&mut self, x: u16, nnn: u16) {
        self.pc = nnn + self.v_register[x as usize] as u16;
    }

    // Get random number and binary AND with NN, and put in VX
    fn op_cxnn(&mut self, x: u16, nn: u16) {
        self.v_register[x as usize] = rand::thread_rng().gen_range(0..=255) & nn as u8;
    }

    // Draw
    fn op_dxyn(&mut self, x: u16, y: u16, n: u16) {
        let x_coord = self.v_register[x as usize];
        let y_coord = self.v_register[y as usize];
        let mut pixel = 0 as u16;

        self.v_register[0xF] = 0;
        for y_line in 0..n {
            pixel = self.memory[(self.i_register + y_line) as usize] as u16;
            for x_line in 0..(8 as u16) {
                if (pixel & (0x80 >> x_line)) != 0 {
                    // Check collision
                    if self.screen[(x_coord as u16 + x_line + ((y_coord as u16 + y_line) * 64)) as usize] == true {
                        self.v_register[0xF] = 1;
                    }
                    self.screen[(x_coord as u16 + x_line + ((y_coord as u16 + y_line) * 64)) as usize] ^= true;
                }
            }
        }
        self.draw_flag = true;
    }

    // Skip next instruction if key is pressed
    fn op_ex9e(&mut self, x: u16) {
        if self.key[self.v_register[x as usize] as usize] {
            self.pc += 2;
        }
    }

    // Skip next instruction if key is not pressed
    fn op_exa1(&mut self, x: u16) {
        if !(self.key[self.v_register[x as usize] as usize]) {
            self.pc += 2;
        }
    }

    // Set VX to delay timer
    fn op_fx07(&mut self, x: u16) {
        self.v_register[x as usize] = self.delay_timer;
    }

    // Halt all instructions until key is pressed
    fn op_fx0a(&mut self, x: u16) {
        // TODO: Probably not the best way to do this
        if self.key == [false; 16] {
            self.pc -= 2;
        }
    }

    // Set delay timer to VX
    fn op_fx15(&mut self, x: u16) {
        self.delay_timer = self.v_register[x as usize];
    }

    // Set sound timer to VX
    fn op_fx18(&mut self, x: u16) {
        self.sound_timer = self.v_register[x as usize];
    }

    // Add VX to I
    fn op_fx1e(&mut self, x: u16) {
        self.i_register += self.v_register[x as usize] as u16;
    }

    // Set I to the location of sprite address for character in VX
    fn op_fx29(&mut self, x: u16) {
        self.i_register = (self.v_register[x as usize] * 5) as u16
    }

    // Store binary-coded decimal of VX, with hundredth digit at memory location I, tenth at I+1, ones at I+2.
    // Ex. If VX is 123, address I would be 1, address I+2 would be 2, address I+3 would be 3.
    fn op_fx33(&mut self, x: u16) {
        self.memory[self.i_register as usize] = self.v_register[x as usize] / 100;
        self.memory[(self.i_register + 1) as usize] = self.v_register[x as usize] % 100 / 10;
        self.memory[(self.i_register + 2) as usize] = self.v_register[x as usize] % 10;
    }

    // Store registers to memory
    //TODO This instruction is ambiguous, and only works on "modern' programs
    fn op_fx55(&mut self, x: u16) {
        for val in 0..=x {
            self.memory[(self.i_register + val) as usize] = self.v_register[val as usize];
        }
    }

    // Load value from memory to registers
    //TODO This instruction is ambiguous, and only works on "modern' programs
    fn op_fx65(&mut self, x: u16) {
        for val in 0..=x {
            self.v_register[val as usize] =  self.memory[(self.i_register + val) as usize];
        }
    }
}