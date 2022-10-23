use std::fmt::{Debug, Display, Formatter};

use crate::{Error, Result};
use crate::machine::instruction::args::{self, DrawArgs, RegisterArgs, AddressArgs, SetArgs, Source, Target};
use crate::machine::instruction::{Flow, Graphics};
use crate::machine::types::{Register, Word};

use super::Instruction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpCode(pub Word);

impl OpCode {
    pub fn bytes(&self) -> [u8; 2] {
        self.0.0.to_be_bytes()
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        let op_code = match instruction {
            Instruction::Exit => 0x00F0,
            Instruction::Graphics(graphics) => match graphics {
                Graphics::Clear => 0x00E0,
                Graphics::Draw { args } => {
                    let upper_byte = 0xD0 | u8::from(&args.x);
                    let lower_byte = u8::from(&args.y).rotate_left(4) | u8::from(&args.height);
                    u16::from_be_bytes([upper_byte, lower_byte])
                }
            }
            Instruction::Flow(flow) => match flow {
                Flow::Return => 0x00EE,
                Flow::Call { args } => 0x2000 | (u16::from(&args.address) & 0x0FFF),
                Flow::Jump { args } => 0x1000 | (u16::from(&args.address) & 0x0FFF),
            }
            Instruction::IndexSet { args } => 0xA000 | (u16::from(&args.address) & 0x0FFF),
            Instruction::Set { args } => {
                match &args.target {
                    Target::Register(vx) => match &args.source {
                        Source::Byte(byte) =>
                            0x6000 | u16::from_be_bytes([vx.into(), byte.into()]),
                        Source::Register(vy) =>
                            0x8000 | u16::from_be_bytes([vx.into(), u8::from(vy).rotate_left(4)]),
                    }
                    Target::Timer(timer) => {
                        let lower_byte: u8 = match timer {
                            args::Timer::Delay => 0x15,
                            args::Timer::Sound => 0x18,
                        };
                        let upper_byte: u8 = match &args.source {
                            Source::Byte(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                            Source::Register(vx) => 0xF0u8 | u8::from(vx),
                        };
                        u16::from_be_bytes([upper_byte, lower_byte])
                    }
                }
            }
            Instruction::Add { args } => {
                match &args.target {
                    Target::Register(vx) => match &args.source {
                        Source::Byte(byte) =>
                            0x7000 | u16::from_be_bytes([vx.into(), byte.into()]),
                        Source::Register(vy) => {
                            let lower = u8::from(vy).rotate_left(4) | 0x04;
                            0x8000 | u16::from_be_bytes([vx.into(), lower])
                        }
                    }
                    Target::Timer(_) => {
                        panic!("not implemented");
                    }
                }
            }
            // todo: deduplicate
            Instruction::GetTimer { args } => 0xF007 | u16::from_be_bytes([u8::from(&args.register), 0]),
            Instruction::Font { args } => 0xF029 | u16::from_be_bytes([u8::from(&args.register), 0]),
            Instruction::KeyAwait { args } => 0xF00A | u16::from_be_bytes([u8::from(&args.register), 0]),
        };
        Ok(OpCode(op_code.into()))
    }
}

impl TryFrom<OpCode> for Instruction {
    type Error = Error;

    fn try_from(opcode: OpCode) -> Result<Self> {
        let (highest, rest) = (opcode.0.0 & 0xF000, opcode.0.0 & 0x0FFF);
        match highest {
            0 => match rest {
                0x0E0 => Ok(Instruction::Graphics(Graphics::Clear)),
                0x0EE => Ok(Instruction::Flow(Flow::Return)),
                0x0F0 => Ok(Instruction::Exit),
                _ => Err(Error::InvalidOpCode(opcode)),
            },
            0x1000 | 0x2000 | 0xA000 => {
                let args = AddressArgs { address: rest.try_into()? };
                Ok(match highest >> 12 {
                    0x1 => Instruction::Flow(Flow::Jump { args }),
                    0x2 => Instruction::Flow(Flow::Call { args }),
                    0xA => Instruction::IndexSet { args },
                    _ => panic!("how did we get here?!"),
                })
            }
            0x6000 | 0x7000 => {
                let [register, lower_byte] = rest.to_be_bytes();
                let args = SetArgs {
                    source: Source::Byte(lower_byte.into()),
                    target: Target::Register(register.try_into()?),
                    // for assignment 0x6000 carry, doesn't matter, but addition 0x7000 is the one
                    // arithmetic operation on CHIP-8 for which the carry bit should not be set
                    carry: false,
                };
                Ok(if highest == 0x7000 { Instruction::Add { args } } else { Instruction::Set { args } })
            }
            0x8000 if rest & 0x00F == 0 || rest & 0x00F == 4 => {
                let [vx, vy] = rest.to_be_bytes();
                let args = SetArgs {
                    source: Source::Register((vy & 0xF0).rotate_right(4).try_into()?),
                    target: Target::Register(vx.try_into()?),
                    carry: true,
                };
                Ok(if rest & 0x00F == 4 { Instruction::Add { args } } else { Instruction::Set { args } })
            }
            0xD000 => {
                let [vx, lower] = rest.to_be_bytes();
                let vy = lower.rotate_left(4) & 0x0F;
                let height = lower & 0x0F;
                Ok(Instruction::Graphics(Graphics::Draw {
                    args: DrawArgs {
                        x: vx.try_into()?,
                        y: vy.try_into()?,
                        height: height.try_into()?,
                    }
                }))
            }
            0xF000 => {
                let register = Register::try_from((rest & 0x0F00).to_be_bytes()[0])?;
                match rest & 0x00FF {
                    0x0A => Ok(Instruction::KeyAwait { args: RegisterArgs { register } }),
                    0x07 => Ok(Instruction::GetTimer { args: RegisterArgs { register } }),
                    0x29 => Ok(Instruction::Font { args: RegisterArgs { register } }),
                    byte @ (0x15 | 0x18) => {
                        let target = Target::Timer(if byte == 0x15 { args::Timer::Delay } else { args::Timer::Sound });
                        let source = Source::Register(register);
                        // todo: different args for this? presence of carry flag here is confusing
                        Ok(Instruction::Set { args: SetArgs { target, source, carry: true } })
                    }
                    _ => Err(Error::InvalidOpCode(opcode)),
                }
            }
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}