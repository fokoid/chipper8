use std::fmt::{Debug, Display, Formatter};

use crate::{Error, Result};
use crate::machine::instruction::args::{self, DrawArgs, RegisterArgs, SetAddressArgs, SetArgs, Source, Target};
use crate::machine::types::Register;

use super::Instruction;

pub struct OpCode(pub u16);

impl OpCode {
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
                Instruction::Jump { args } => 0x1000 | (u16::from(&args.address) & 0x0FFF),
                Instruction::IndexSet { args } => 0xA000 | (u16::from(&args.address) & 0x0FFF),
                Instruction::Set { args: SetArgs { source, target } } => {
                    match &target {
                        Target::Register(vx) => match &source {
                            Source::Byte(byte) =>
                                0x6000 | u16::from_be_bytes([vx.into(), byte.into()]),
                            Source::Register(_vy) => todo!(),
                        }
                        Target::Timer(timer) => {
                            let lower_byte: u8 = match timer {
                                args::Timer::Delay => 0x15,
                                args::Timer::Sound => 0x18,
                            };
                            let upper_byte: u8 = match &source {
                                Source::Byte(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                                Source::Register(vx) => 0xF0u8 | u8::from(vx),
                            };
                            u16::from_be_bytes([upper_byte, lower_byte])
                        }
                    }
                }
                Instruction::Add { args: SetArgs { source, target } } => {
                    match &target {
                        Target::Register(vx) => match &source {
                            Source::Byte(byte) =>
                                0x7000 | u16::from_be_bytes([vx.into(), byte.into()]),
                            Source::Register(_vy) => todo!(),
                        }
                        Target::Timer(_) => {
                            panic!("not implemented");
                        }
                    }
                }
                Instruction::Draw { args } => {
                    let upper_byte = 0xD0 | u8::from(&args.x);
                    let lower_byte = u8::from(&args.y).rotate_left(4) | u8::from(&args.height);
                    u16::from_be_bytes([upper_byte, lower_byte])
                }
                // todo: deduplicate
                Instruction::GetTimer { args } => 0xF007 | u16::from_be_bytes([u8::from(&args.register), 0]),
                Instruction::Font { args } => 0xF029 | u16::from_be_bytes([u8::from(&args.register), 0]),
                Instruction::KeyAwait { args } => 0xF00A | u16::from_be_bytes([u8::from(&args.register), 0]),
            }
        ))
    }
}

impl TryFrom<&OpCode> for Instruction {
    type Error = Error;

    fn try_from(opcode: &OpCode) -> Result<Self> {
        match opcode.0 & 0xF000 {
            0 => match opcode.0 & 0x0FFF {
                0x0E0 => Ok(Instruction::ClearScreen),
                0x0F0 => Ok(Instruction::Exit),
                _ => Err(Error::InvalidOpCode(OpCode(opcode.0))),
            },
            first @ (0x1000 | 0xA000) => {
                let args = SetAddressArgs { address: (opcode.0 & 0x0FFF).try_into()? };
                Ok(if first == 0x1000 { Instruction::Jump { args } } else { Instruction::IndexSet { args } })
            }
            first @ (0x6000 | 0x7000) => {
                let [register, lower_byte] = (opcode.0 & 0x0FFF).to_be_bytes();
                let args = SetArgs {
                    source: Source::Byte(lower_byte.into()),
                    target: Target::Register(register.try_into()?),
                };
                Ok(if first == 0x6000 { Instruction::Set { args } } else { Instruction::Add { args } })
            }
            0xD000 => {
                let [vx, lower] = (opcode.0 & 0xFFF).to_be_bytes();
                let vy = lower.rotate_left(4) & 0x0F;
                let height = lower & 0x0F;
                Ok(Instruction::Draw {
                    args: DrawArgs {
                        x: vx.try_into()?,
                        y: vy.try_into()?,
                        height: height.try_into()?,
                    }
                })
            }
            0xF000 => {
                let register = Register::try_from((opcode.0 & 0x0F00).to_be_bytes()[0])?;
                match opcode.0 & 0x00FF {
                    0x0A => Ok(Instruction::KeyAwait { args: RegisterArgs { register } }),
                    0x07 => Ok(Instruction::GetTimer { args: RegisterArgs { register } }),
                    0x29 => Ok(Instruction::Font { args: RegisterArgs { register } }),
                    byte @ (0x15 | 0x18) => {
                        let target = Target::Timer(if byte == 0x15 { args::Timer::Delay } else { args::Timer::Sound });
                        let source = Source::Register(register);
                        Ok(Instruction::Set { args: SetArgs { target, source } })
                    }
                    _ => Err(Error::InvalidOpCode(OpCode(opcode.0))),
                }
            }
            _ => Err(Error::InvalidOpCode(OpCode(opcode.0))),
        }
    }
}