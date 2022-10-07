use std::ops::RangeInclusive;

pub const MEMORY_SIZE: usize = 4096;
pub const NUM_REGISTERS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const FONT_RANGE: RangeInclusive<usize> = 0x050..=0x09F;

const FONT_GLYPHS: [u8; 80] = [
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

// TODO: restrict pointers to 12 bits at compile time?
type Pointer = usize;

type Timer = u8;

#[derive(Debug)]
struct Stack {
    data: [u16; STACK_SIZE],
    pointer: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            pointer: 0,
        }
    }

    fn pop(&mut self) -> u16 {
        if self.pointer == 0 {
            panic!("pop() on empty stack");
        };
        let value = self.data[self.pointer];
        self.pointer -= 1;
        value
    }

    fn push(&mut self, value: u16) {
        if self.pointer == STACK_SIZE {
            panic!("push({}) on full stack", value);
        }
        self.data[self.pointer] = value;
        self.pointer += 1;
    }
}

#[derive(Debug)]
pub struct Machine {
    memory: [u8; MEMORY_SIZE],
    stack: Stack,
    display: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pc: Pointer,
    index: Pointer,
    delay_timer: Timer,
    sound_timer: Timer,
    registers: [u8; NUM_REGISTERS],
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: [0; MEMORY_SIZE],
            stack: Stack::new(),
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            pc: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; NUM_REGISTERS],
        };
        machine.memory[FONT_RANGE].clone_from_slice(&FONT_GLYPHS);
        machine
    }
}