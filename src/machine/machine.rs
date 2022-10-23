use serde::{Deserialize, Serialize};

use crate::{Error, Result};
use crate::assembler::Tokens;
use crate::ui::Rom;

use super::config;
use super::draw_options::DrawOptions;
use super::instruction::{Instruction, OpCode};
use super::instruction::args::{self, Source, Target};
use super::stack::Stack;
use super::types::{Pointer, Timer};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Machine {
    pub registers: Vec<u8>,
    pub stack: Stack,
    pub memory: Vec<u8>,
    pub display: Vec<u8>,
    pub program_counter: Pointer,
    pub index: Pointer,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
    pub key_buffer: Option<u8>,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: vec![0; config::MEMORY_SIZE],
            stack: Stack::new(),
            display: vec![0; config::DISPLAY_SIZE],
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: vec![0; config::NUM_REGISTERS],
            key_buffer: None,
        };
        machine.memory[config::FONT_RANGE].clone_from_slice(&config::FONT_GLYPHS);
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
        self.memory[config::FONT_RANGE].clone_from_slice(&config::FONT_GLYPHS);
    }

    pub fn load(&mut self, offset: Pointer, data: &[u8]) {
        let overflow = if offset + data.len() > self.memory.len() {
            offset + data.len() - self.memory.len()
        } else { 0 };
        let data = &data[..data.len() - overflow];
        self.memory[offset..offset + data.len()].clone_from_slice(data);
    }

    pub fn load_rom(&mut self, rom: &mut Rom, address: Pointer) {
        if rom.loaded_at.is_some() {
            panic!("rom already loaded");
        }
        rom.loaded_at = Some(address);
        self.program_counter = address;
        self.load(self.program_counter, &rom.bytes);
    }

    pub fn unload_rom(&mut self, rom: &mut Rom) {
        if rom.loaded_at.is_none() {
            panic!("attempt to unload ROM that was never loaded");
        }
        self.memory[rom.loaded_range().unwrap()].fill(0);
        rom.loaded_at = None;
        // todo: should we move program counter?
    }

    pub fn demo(&mut self) -> Result<()> {
        self.program_counter = 1000;
        self.memory[self.program_counter] = 0x00E0;
        self.stack.push(0xAAA);
        self.stack.push(0xBBB);
        // put some instructions at these stack addresses show they show in the visualization
        self.set_instruction_at_address(0xAAA, &Instruction::ClearScreen)?;
        self.set_instruction_at_address(0xBBB, &Instruction::try_from(Tokens::from("font 0x3")).unwrap())?;
        self.registers[0] = 0x12;
        self.registers[1] = 0xAB;
        self.delay_timer = 0xF;
        self.sound_timer = 1;
        self.display[1000] = 0xFF;
        Ok(())
    }

    pub fn byte_at_address(&self, address: usize) -> Option<u8> {
        Some(*self.memory.get(address)?)
    }

    pub fn word_at_address(&self, address: usize) -> Option<u16> {
        let bytes = [self.byte_at_address(address)?, self.byte_at_address(address + 1)?];
        Some(u16::from_be_bytes(bytes))
    }

    pub fn instruction_at_address(&self, address: usize) -> Result<Instruction> {
        let opcode = OpCode(self.word_at_address(address).unwrap_or(0));
        Instruction::try_from(&opcode)
    }

    fn set_instruction_at_address(&mut self, address: usize, instruction: &Instruction) -> Result<()> {
        let opcode: OpCode = instruction.try_into()?;
        self.memory[address..address + 2].clone_from_slice(&opcode.bytes());
        Ok(())
    }

    pub fn at_program_counter(&self) -> Option<u16> {
        self.word_at_address(self.program_counter)
    }

    pub fn next_instruction(&self) -> Result<Instruction> {
        self.instruction_at_address(self.program_counter)
    }

    pub fn at_index(&self) -> Option<u8> {
        self.byte_at_address(self.index)
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::Exit => { return Err(Error::MachineExit); }
            Instruction::ClearScreen => self.display.fill(0),
            Instruction::Jump { args } => {
                self.program_counter = (&args.address).into();
            }
            Instruction::IndexSet { args } => {
                self.index = (&args.address).into();
            }
            Instruction::Set { args } | Instruction::Add { args } => {
                let source = match &args.source {
                    Source::Byte(x) => x.into(),
                    Source::Register(r) => self.registers[usize::from(r)],
                };
                let target = match &args.target {
                    Target::Timer(args::Timer::Delay) => &mut self.delay_timer,
                    Target::Timer(args::Timer::Sound) => &mut self.sound_timer,
                    Target::Register(r) => &mut self.registers[usize::from(r)],
                };
                if let Instruction::Set { args: _ } = &instruction {
                    *target = source;
                } else {
                    *target += source;
                }
            }
            Instruction::Draw { args } => {
                let x = self.registers[usize::from(&args.x)] as usize % config::DISPLAY_WIDTH;
                let y = self.registers[usize::from(&args.y)] as usize % config::DISPLAY_HEIGHT;
                DrawOptions::new(
                    &self.memory[self.index..self.index + u8::from(&args.height) as usize],
                    &mut self.display,
                    [config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT],
                ).at([x, y]).draw();
            }
            Instruction::Font { args } => {
                let char = self.registers[usize::from(&args.register)] as usize & 0x0F;
                self.index = config::FONT_RANGE.start + config::FONT_SPRITE_HEIGHT * char;
            }
            Instruction::GetTimer { args } => {
                self.registers[usize::from(&args.register)] = self.delay_timer;
            }
            Instruction::KeyAwait { args } => {
                if let Some(key) = self.key_buffer {
                    self.registers[usize::from(&args.register)] = key;
                } else {
                    self.program_counter -= 2;
                }
            }
        };
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        let instruction = self.next_instruction().unwrap();
        self.sound_timer -= if self.sound_timer > 0 { 1 } else { 0 };
        self.delay_timer -= if self.delay_timer > 0 { 1 } else { 0 };
        self.program_counter += 2;
        self.execute(&instruction)?;
        Ok(())
    }
}
