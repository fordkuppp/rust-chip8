// Reference: https://aquova.net/chip8/chip8.pdf, https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

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
    pub memory: [u8; 4096],     // memory
    pub pc: u16,                // program counter
    pub i: u16,                 // index register
    pub register: [u8; 16],     // register (V0 to VF)
    pub stack: [u16; 16],       // LIFO stack
    pub stack_pt: u8,           // stack pointer
    pub delay_timer: u8,        // delay timer
    pub sound_timer: u8,        // sound timer
    pub opcode: u16,            // current opcode
    pub keypad: [bool; 16],     // keypad state
    pub screen: [bool; 64*32]   // screen state
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut new_chip8 = Chip8 {
            memory: [0; 4096],
            pc: 0x200,          // start at 0x200 per original chip-8
            i: 0,
            register: [0; 16],
            stack: [0; 16],
            stack_pt: 0,
            delay_timer: 0,
            sound_timer: 0,
            opcode: 0,
            keypad: [false; 16] ,
            screen: [false; 64*32] ,
        };
        new_chip8.memory[0x050..0x09F].copy_from_slice(&FONT);
        new_chip8
    }

    fn push(&mut self, val: u16) {
        self.stack[self.stack_pt] = val;
        self.stack_pt += 1;
    }

    fn pop(&mut self) {
        self.stack_pt -= 1;
        self.stack[self.stack_pt];
    }
}