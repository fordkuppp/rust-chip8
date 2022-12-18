// Reference: https://aquova.net/chip8/chip8.pdf

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
        Chip8 {
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
        }
    }
}