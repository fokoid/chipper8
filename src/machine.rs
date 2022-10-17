use std::cmp::min;
use std::ops::RangeInclusive;
use std::time::Duration;

use crate::instructions::{self, Instruction, OpCode};

pub const MEMORY_SIZE: usize = 4096;
pub const NUM_REGISTERS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const FONT_RANGE: RangeInclusive<usize> = 0x050..=0x09F;
pub const FONT_SPRITE_HEIGHT: usize = 5;
pub const FRAMES_PER_SECOND: u64 = 60;
pub const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / FRAMES_PER_SECOND);

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
pub type Pointer = usize;

type Timer = u8;

#[derive(Debug)]
pub struct Stack {
    pub data: [u16; STACK_SIZE],
    pub pointer: usize,
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

    fn reset(&mut self) {
        self.data.fill(0);
        self.pointer = 0;
    }
}

#[derive(Debug)]
pub struct Machine {
    pub registers: [u8; NUM_REGISTERS],
    pub stack: Stack,
    pub memory: [u8; MEMORY_SIZE],
    pub display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub program_counter: Pointer,
    pub index: Pointer,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: [0; MEMORY_SIZE],
            stack: Stack::new(),
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; NUM_REGISTERS],
        };
        machine.memory[FONT_RANGE].clone_from_slice(&FONT_GLYPHS);
        machine
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.stack.reset();
        self.display.fill(0);
        self.program_counter = 0;
        self.index = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.registers.fill(0);
        self.memory[FONT_RANGE].clone_from_slice(&FONT_GLYPHS);
    }

    pub fn load(&mut self, offset: Pointer, data: &[u8]) {
        let overflow = if offset + data.len() > self.memory.len() {
            offset + data.len() - self.memory.len()
        } else { 0 };
        let data = &data[..data.len() - overflow];
        self.memory[offset..offset + data.len()].clone_from_slice(data);
    }

    pub fn demo(&mut self) {
        self.program_counter = 1000;
        self.memory[self.program_counter] = 0x00E0;
        self.stack.push(0xAAA);
        self.stack.push(0xBBB);
        // put some instructions at these stack addresses show they show in the visualization
        self.set_instruction_at_address(0xAAA, &Instruction::ClearScreen);
        self.set_instruction_at_address(0xBBB, &Instruction::Font(3));
        self.registers[0] = 0x12;
        self.registers[1] = 0xAB;
        self.delay_timer = 0xF;
        self.sound_timer = 1;
        self.display[1000] = 0xFF;
    }

    pub fn byte_at_address(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn word_at_address(&self, address: usize) -> u16 {
        u16::from_be_bytes(self.memory[address..address + 2].try_into().unwrap())
    }

    pub fn instruction_at_address(&self, address: usize) -> instructions::Result<Instruction> {
        OpCode(self.word_at_address(address)).as_instruction()
    }

    fn set_instruction_at_address(&mut self, address: usize, instruction: &Instruction) {
        let opcode: OpCode = instruction.into();
        self.memory[address..address + 2].clone_from_slice(&opcode.bytes())
    }

    pub fn at_program_counter(&self) -> u16 {
        self.word_at_address(self.program_counter)
    }

    pub fn next_instruction(&self) -> instructions::Result<Instruction> {
        OpCode(self.at_program_counter()).as_instruction()
    }

    pub fn at_index(&self) -> u8 {
        self.byte_at_address(self.index)
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::ClearScreen => self.display.fill(0),
            Instruction::Jump(address) => {
                self.program_counter = *address as Pointer;
            }
            Instruction::Set(register, value) => {
                self.registers[*register as usize] = *value;
            }
            Instruction::Add(register, value) => {
                self.registers[*register as usize] += *value;
            }
            Instruction::IndexSet(value) => {
                self.index = *value as Pointer;
            }
            Instruction::Draw(vx, vy, height) => {
                let [x, y] = [self.registers[*vx as usize] as usize % DISPLAY_WIDTH, self.registers[*vy as usize] as usize % DISPLAY_HEIGHT];
                DrawOptions::new(
                    &self.memory[self.index..self.index + *height as usize],
                    &mut self.display,
                    [DISPLAY_WIDTH, DISPLAY_HEIGHT],
                ).at([x, y]).draw();
            }
            Instruction::Font(register) => {
                let char = self.registers[*register as usize] as usize & 0x0F;
                self.index = FONT_RANGE.start() + FONT_SPRITE_HEIGHT * char;
            }
            Instruction::TimerSound(value) => {
                self.sound_timer = *value;
            }
        }
    }

    pub fn step(&mut self) -> instructions::Result<()> {
        let instruction = self.next_instruction().unwrap();
        self.program_counter += 2;
        self.execute(&instruction);
        Ok(())
    }
}

pub struct DrawOptions<'a> {
    pos: [usize; 2],
    display_size: [usize; 2],
    source: &'a [u8],
    target: &'a mut [u8],
}

impl<'a> DrawOptions<'a> {
    pub fn new(source: &'a [u8], target: &'a mut [u8], display_size: [usize; 2]) -> Self {
        Self {
            pos: [0, 0],
            display_size,
            source,
            target,
        }
    }

    pub fn at(mut self, pos: [usize; 2]) -> Self {
        self.pos = pos;
        self
    }

    pub fn draw(self) {
        let bytes = self.source;
        let [x, y] = self.pos;
        let [display_width, display_height] = self.display_size;
        let height = self.source.len();
        for j in y..min(y + height, display_height) {
            let mut byte = bytes[j - y];
            for i in x..min(x + 8, display_width) {
                self.target[i + j * display_width] ^= if byte & 0b10000000 != 0 { 0xFF } else { 0 };
                byte = byte.rotate_left(1);
            }
        };
    }
}