use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

// todo: everywhere use types from machine::types here
use serde::{Deserialize, Serialize};

use crate::{Error, Result};
use crate::assembler::Tokens;
use crate::machine::instruction::Input;
use crate::ui::Rom;

use super::config;
use super::draw_options::DrawOptions;
use super::instruction::{Flow, Graphics, Instruction, Memory, OpCode};
use super::instruction::args::{self, BinaryOp, BinaryOpArgs, Comparator, IndexOp, IndexOpArgs, IndexSource, Source, Target};
use super::stack::Stack;
use super::types::{Address, Register, Timer};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MachineConfig {
    pub bitshift_ignore_y: bool,
    pub jump_xnn: bool,
    pub load_increment_index: bool,
    pub auto_exit: bool,
}

impl MachineConfig {
    pub fn new() -> Self {
        Self {
            // new (incorrect) behaviour required for BC_test ROM to pass
            bitshift_ignore_y: true,
            jump_xnn: false,
            load_increment_index: false,
            // terminate execution when an infinite loop is hit (useful e.g. for integration tests
            // where we want to stop and check output when the test ROM hits its infinite loop)
            auto_exit: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Machine {
    pub registers: Vec<u8>,
    pub stack: Stack,
    pub memory: Vec<u8>,
    pub display: Vec<u8>,
    pub program_counter: Address,
    pub index: Address,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
    pub key_buffer: Option<u8>,
    pub config: MachineConfig,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Self {
            memory: vec![0; config::MEMORY_SIZE],
            stack: Stack::new(),
            display: vec![0; config::DISPLAY_SIZE],
            program_counter: Address::new(),
            index: Address::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: vec![0; config::NUM_REGISTERS],
            key_buffer: None,
            config: MachineConfig::new(),
        };
        machine.memory[config::FONT_RANGE].clone_from_slice(&config::FONT_GLYPHS);
        machine
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.stack.reset();
        self.display.fill(0);
        self.program_counter = Address::new();
        self.index = Address::new();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.registers.fill(0);
        self.memory[config::FONT_RANGE].clone_from_slice(&config::FONT_GLYPHS);
    }

    pub fn load(&mut self, offset: &Address, data: &[u8]) {
        let offset = usize::from(offset);
        let overflow = if offset + data.len() > self.memory.len() {
            offset + data.len() - self.memory.len()
        } else { 0 };
        let data = &data[..data.len() - overflow];
        self.memory[offset..offset + data.len()].clone_from_slice(data);
    }

    pub fn load_rom(&mut self, rom: &mut Rom, address: Option<&Address>) {
        if rom.loaded_at.is_some() {
            panic!("rom already loaded");
        }
        let default_load_address = Address::try_from(0x200u16).unwrap();
        let address = address.unwrap_or(&default_load_address);
        rom.loaded_at = Some(usize::from(address));
        self.load(&address, &rom.bytes);
        self.program_counter = address.clone();
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
        self.program_counter = 1000u16.try_into().unwrap();
        self.memory[usize::from(&self.program_counter)] = 0x00E0;
        self.stack.push(0xAAAu16.try_into().unwrap());
        self.stack.push(0xBBBu16.try_into().unwrap());
        // put some instructions at these stack addresses show they show in the visualization
        self.set_instruction_at_address(&Address::try_from(0xAAAu16).unwrap(), &Instruction::Graphics(Graphics::Clear))?;
        self.set_instruction_at_address(&Address::try_from(0xBBBu16).unwrap(), &Instruction::try_from(Tokens::from("font V3")).unwrap())?;
        self.registers[0] = 0x12;
        self.registers[1] = 0xAB;
        self.delay_timer = 0xF;
        self.sound_timer = 1;
        self.display[1000] = 0xFF;
        Ok(())
    }

    pub fn byte_at_address(&self, address: &Address) -> Option<u8> {
        Some(*self.memory.get(usize::from(address))?)
    }

    pub fn word_at_address(&self, address: &Address) -> Option<u16> {
        let mut next_address = address.clone();
        next_address.advance(1u8.into());
        let bytes = [self.byte_at_address(address)?, self.byte_at_address(&next_address)?];
        Some(u16::from_be_bytes(bytes))
    }

    pub fn instruction_at_address(&self, address: &Address) -> Result<Instruction> {
        let opcode = OpCode(self.word_at_address(address).unwrap_or(0).into());
        Instruction::try_from(opcode)
    }

    fn set_instruction_at_address(&mut self, address: &Address, instruction: &Instruction) -> Result<()> {
        let opcode: OpCode = instruction.try_into()?;
        self.memory[address.as_range(2)].clone_from_slice(&opcode.bytes());
        Ok(())
    }

    pub fn at_program_counter(&self) -> Option<u16> {
        self.word_at_address(&self.program_counter)
    }

    pub fn next_instruction(&self) -> Result<Instruction> {
        self.instruction_at_address(&self.program_counter)
    }

    pub fn at_index(&self) -> Option<u8> {
        self.byte_at_address(&self.index)
    }

    fn read_source(&self, source: &Source) -> u8 {
        match &source {
            Source::Byte(x) => x.into(),
            Source::Register(r) => self.registers[usize::from(r)],
            Source::Timer(timer) => match timer {
                args::Timer::Delay => self.delay_timer,
                args::Timer::Sound => self.sound_timer,
            }
        }
    }

    fn set_flag(&mut self, value: u8) { self.registers[0xF] = value; }

    fn execute_graphics(&mut self, graphics: &Graphics) -> Result<()> {
        match graphics {
            Graphics::Clear => { self.display.fill(0); }
            Graphics::Draw { args } => {
                let x = self.registers[usize::from(&args.x)] as usize % config::DISPLAY_WIDTH;
                let y = self.registers[usize::from(&args.y)] as usize % config::DISPLAY_HEIGHT;
                let index_start = usize::from(&self.index);
                let index_end = index_start + usize::from(&args.height);
                self.registers[0xF] = if DrawOptions::new(
                    &self.memory[index_start..index_end],
                    &mut self.display,
                    [config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT],
                ).at([x, y]).draw() { 1 } else { 0 };
            }
        };
        Ok(())
    }

    fn execute_flow(&mut self, flow: &Flow) -> Result<()> {
        match flow {
            Flow::Return => {
                self.program_counter = self.stack.pop().into();
            }
            Flow::Jump { args } | Flow::Call { args } | Flow::Sys { args } => {
                let mut address = args.address.clone();
                if let Some(register) = &args.register.as_ref().map(|register| if self.config.jump_xnn {
                    let [upper, _] = u16::from(&args.address).to_be_bytes();
                    // we know upper byte of a u12 is only a single nibble
                    Register::try_from(upper).unwrap()
                } else { register.clone() }) {
                    address.advance(self.registers[usize::from(register)].into());
                }
                match flow {
                    Flow::Jump { args: _ } => { self.program_counter = address; }
                    Flow::Call { args: _ } => {
                        // todo: can we swap here?
                        self.stack.push(self.program_counter.clone());
                        self.program_counter = address;
                    }
                    Flow::Sys { args: _ } => {
                        panic!("not implemented: sys call");
                    }
                    Flow::Return | Flow::Branch { args: _ } => {
                        // todo: can we avoid this?
                        panic!("how did we get here?");
                    }
                }
            }
            Flow::Branch { args } => {
                // todo extract logic for 'get value of Source'
                let lhs: u8 = self.read_source(&args.lhs);
                let rhs: u8 = self.read_source(&args.rhs);
                if match &args.comparator {
                    Comparator::Equal => lhs == rhs,
                    Comparator::NotEqual => lhs != rhs,
                } {
                    self.program_counter.step();
                };
            }
        };
        Ok(())
    }

    fn execute_arithmetic(&mut self, args: &BinaryOpArgs) -> Result<()> {
        {
            let source = self.read_source(&args.source);
            let target = match &args.target {
                Target::Timer(args::Timer::Delay) => &mut self.delay_timer,
                Target::Timer(args::Timer::Sound) => &mut self.sound_timer,
                Target::Register(r) => &mut self.registers[usize::from(r)],
            };
            match &args.op {
                BinaryOp::Assign => {
                    *target = source;
                }
                BinaryOp::Add => {
                    let (result, carry_flag) = target.overflowing_add(source);
                    *target = result;
                    self.set_flag(carry_flag.into());
                }
                // todo: deduplicate with BinaryOp::Add?
                BinaryOp::AddWrapping => {
                    let (result, _carry_flag) = target.overflowing_add(source);
                    *target = result;
                }
                BinaryOp::Subtract => {
                    let (result, carry_flag) = target.overflowing_sub(source);
                    *target = result;
                    self.set_flag(1 - u8::from(carry_flag));
                }
                // todo: deduplicate with BinaryOp::Subtract?
                BinaryOp::SubtractAlt => {
                    let (result, carry_flag) = source.overflowing_sub(*target);
                    *target = result;
                    self.set_flag(1 - u8::from(carry_flag));
                }
                BinaryOp::BitAnd => {
                    target.bitand_assign(source);
                }
                BinaryOp::BitOr => {
                    target.bitor_assign(source);
                }
                BinaryOp::BitXor => {
                    target.bitxor_assign(source);
                }
                BinaryOp::BitShiftLeft => {
                    if !self.config.bitshift_ignore_y {
                        *target = source;
                    }
                    let highest_bit: u8 = *target / 128;
                    target.shl_assign(1);
                    self.set_flag(highest_bit);
                }
                BinaryOp::BitShiftRight => {
                    if !self.config.bitshift_ignore_y {
                        *target = source;
                    }
                    let lowest_bit = *target & 1;
                    target.shr_assign(1);
                    self.set_flag(lowest_bit);
                }
                BinaryOp::Random => {
                    *target = source & rand::random::<u8>();
                }
            }
        };
        Ok(())
    }

    fn execute_memory(&mut self, memory: &Memory) -> Result<()> {
        let args = match memory {
            Memory::Load { args } | Memory::Save { args } => args
        };
        let last = usize::from(&args.register) + 1;
        let (source, target) = match memory {
            Memory::Load { args: _ } => (&self.memory[self.index.as_range(last)], &mut self.registers[..last]),
            Memory::Save { args: _ } => (&self.registers[..last], &mut self.memory[self.index.as_range(last)]),
        };
        target.clone_from_slice(source);
        if self.config.load_increment_index {
            self.index.advance(last.try_into().unwrap());
        };
        Ok(())
    }

    fn execute_index(&mut self, args: &IndexOpArgs) -> Result<()> {
        let source = match &args.source {
            // todo: can we take ownership of args here to avoid the copy?
            IndexSource::Value(address) => address.clone(),
            IndexSource::Register(vx) => self.registers[usize::from(vx)].into(),
        };
        match &args.op {
            IndexOp::Assign => { self.index = source; }
            IndexOp::Add => { self.index.advance(source.0); }
            IndexOp::AssignFont => {
                let char = usize::from(&source) & 0x0F;
                let index = config::FONT_RANGE.start + config::FONT_SPRITE_HEIGHT * char;
                self.index = Address::try_from(index)?;
            }
        };
        Ok(())
    }

    fn execute_input(&mut self, input: &Input) -> Result<()> {
        match input {
            Input::Await { args } => {
                if let Some(key) = self.key_buffer {
                    self.registers[usize::from(&args.register)] = key;
                } else {
                    self.program_counter.step_back();
                }
            }
            Input::Branch { args } => {
                let result = Some(self.read_source(&args.key)) == self.key_buffer;
                if match &args.comparator {
                    Comparator::Equal => result,
                    Comparator::NotEqual => !result,
                } {
                    self.program_counter.step();
                }
            }
        }
        Ok(())
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::Exit => { return Err(Error::MachineExit); }
            Instruction::Graphics(graphics) => self.execute_graphics(graphics)?,
            Instruction::Flow(flow) => self.execute_flow(flow)?,
            Instruction::Arithmetic { args } => self.execute_arithmetic(args)?,
            Instruction::Memory(memory) => self.execute_memory(memory)?,
            Instruction::Index { args } => self.execute_index(args)?,
            Instruction::Input(input) => self.execute_input(input)?,
            Instruction::BinaryCodedDecimal { args } => {
                let value = self.registers[usize::from(&args.register)];
                let digits = [value / 100 % 10, value / 10 % 10, value % 10];
                self.memory[self.index.as_range(3)].clone_from_slice(&digits);
            }
        };
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        let instruction = self.next_instruction().unwrap();
        if self.config.auto_exit {
            if let Instruction::Flow(Flow::Jump { args }) = &instruction {
                if args.address == self.program_counter && args.register.is_none() {
                    return Err(Error::MachineExit);
                }
            }
        }
        self.sound_timer -= if self.sound_timer > 0 { 1 } else { 0 };
        self.delay_timer -= if self.delay_timer > 0 { 1 } else { 0 };
        self.program_counter.step();
        self.execute(&instruction)?;
        Ok(())
    }
}
