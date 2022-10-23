use std::fmt::{Debug, Display, Formatter};

use ux::u4;

use crate::{Error, Result};
use crate::command::tokens::{Token, Tokens};
use crate::machine::instruction::{self, DrawArgs, SetArgs, Source, Target};

use super::Instruction;

pub struct OpCode(pub u16);

impl OpCode {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        match tokens.next() {
            Some(Token::Hex(s)) => {
                Ok(OpCode(u16::from_str_radix(&s[2..], 16)?))
            }
            x => Err(Error::OpCodeSyntaxError(format!("{:?}", x))),
        }
    }

    pub fn bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

impl Into<String> for OpCode {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl TryFrom<&Instruction> for OpCode {
    type Error = Error;

    fn try_from(instruction: &Instruction) -> Result<Self> {
        Ok(OpCode(
            match instruction {
                Instruction::Exit => 0x00F0,
                Instruction::ClearScreen => 0x00E0,
                Instruction::Jump(address) => 0x1000 | (address & 0x0FFF),
                Instruction::Set { args: SetArgs { source, target } } => {
                    match &target {
                        Target::Register(vx) => match &source {
                            Source::Value(value) =>
                                0x6000 | u16::from_be_bytes([vx.index.into(), *value]),
                            Source::Register(_vy) => todo!(),
                        }
                        Target::Timer(timer) => {
                            let lower_byte: u8 = match timer {
                                instruction::Timer::Delay => 0x15,
                                instruction::Timer::Sound => 0x18,
                            };
                            let upper_byte: u8 = match &source {
                                Source::Value(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                                Source::Register(vx) => 0xF0u8 | u8::from(vx.index),
                            };
                            u16::from_be_bytes([upper_byte, lower_byte])
                        }
                    }
                }
                Instruction::Add(register, value) =>
                    0x7000 | u16::from_be_bytes([*register, *value]),
                Instruction::IndexSet(value) => 0xA000 | (value & 0x0FFF),
                Instruction::Draw{ args: DrawArgs { x, y, height}} =>
                    0xD000 | u16::from_be_bytes([u8::from(x.index), u8::from(y.index).rotate_left(4) | u8::from(*height)]),
                Instruction::TimerGet(register) => 0xF007 | u16::from_be_bytes([*register, 0]),
                Instruction::Font(register) => 0xF029 | u16::from_be_bytes([*register, 0]),
                Instruction::AwaitKey(register) => 0xF00A | u16::from_be_bytes([*register, 0]),
            }
        ))
    }
}

impl OpCode {
    pub fn as_instruction(&self) -> Result<Instruction> {
        match self.0 & 0xF000 {
            0 => match self.0 & 0x0FFF {
                0x0E0 => Ok(Instruction::ClearScreen),
                0x0F0 => Ok(Instruction::Exit),
                _ => Err(Error::InvalidOpCode(OpCode(self.0))),
            },
            0x1000 => Ok(Instruction::Jump(self.0 & 0x0FFF)),
            0x6000 => {
                let [register, value] = (self.0 & 0x0FFF).to_be_bytes();
                Ok(Instruction::Set {
                    args: SetArgs {
                        source: Source::Value(value),
                        target: Target::Register(register.try_into()?),
                    }
                })
            }
            0x7000 => {
                let [register, value] = (self.0 & 0x0FFF).to_be_bytes();
                Ok(Instruction::Add(register, value))
            }
            0xA000 => Ok(Instruction::IndexSet(self.0 & 0x0FFF)),
            0xD000 => {
                let [vx, lower] = (self.0 & 0xFFF).to_be_bytes();
                let vy = lower.rotate_left(4) & 0x0F;
                let height = lower & 0x0F;
                Ok(Instruction::Draw {
                    args: DrawArgs {
                        x: vx.try_into()?,
                        y: vy.try_into()?,
                        height: u4::try_from(height).map_err(|_error| {
                            Error::IntSizeError(String::from("register"), height.into())
                        })?
                    }
                })
            }
            0xF000 => {
                match self.0 & 0x00FF {
                    0x0A => Ok(Instruction::AwaitKey((self.0 & 0x0F00).to_be_bytes()[0])),
                    0x07 => Ok(Instruction::TimerGet((self.0 & 0x0F00).to_be_bytes()[0])),
                    byte @ (0x15 | 0x18) => {
                        let target = Target::Timer(if byte == 0x15 { instruction::Timer::Delay } else { instruction::Timer::Sound });
                        let source = Source::Register((self.0 & 0x0F00).to_be_bytes()[0].try_into()?);
                        Ok(Instruction::Set { args: SetArgs { target, source } })
                    }
                    0x29 => Ok(Instruction::Font((self.0 & 0x0F00).to_be_bytes()[0])),
                    _ => Err(Error::InvalidOpCode(OpCode(self.0))),
                }
            }
            _ => Err(Error::InvalidOpCode(OpCode(self.0))),
        }
    }
}