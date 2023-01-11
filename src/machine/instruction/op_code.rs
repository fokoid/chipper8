use std::fmt::{Debug, Display, Formatter};

use crate::{Error, Result};
use crate::machine::instruction::{Flow, Graphics};
use crate::machine::instruction::args::{BinaryOp, BinaryOpArgs, BranchArgs, Comparator, DrawArgs, IndexOp, IndexOpArgs, IndexSource, JumpArgs, RegisterArgs, Source, Target, Timer};
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
                Flow::Sys { args } | Flow::Call { args } | Flow::Jump { args } => {
                    let rest = u16::from(&args.address) & 0x0FFF;
                    match flow {
                        Flow::Sys { args } => match args.register {
                            Some(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                            None => 0x0000 | rest,
                        }
                        Flow::Jump { args } => match &args.register {
                            None => 0x1000 | rest,
                            Some(Register(x)) if u8::from(x.0) == 0 => 0xB000 | rest,
                            Some(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                        }
                        Flow::Call { args } => match args.register {
                            Some(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                            None => 0x2000 | rest,
                        }
                        Flow::Return | Flow::Branch { args: _ } => {
                            // todo: can we avoid this?
                            panic!("how did we get here?");
                        }
                    }
                }
                Flow::Branch { args } => {
                    let lower_byte: u8 = match &args.rhs {
                        Source::Byte(b) => b.into(),
                        Source::Register(r) => u8::from(r).rotate_left(4),
                        Source::Timer(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                    };
                    let upper_byte: u8 = match &args.rhs {
                        Source::Byte(_) => match &args.comparator {
                            Comparator::Equal => 0x30u8,
                            Comparator::NotEqual => 0x40u8,
                        }
                        Source::Register(_) => match &args.comparator {
                            Comparator::Equal => 0x50u8,
                            Comparator::NotEqual => 0x90u8,
                        }
                        Source::Timer(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                    } | match &args.lhs {
                        Source::Register(r) => u8::from(r),
                        Source::Byte(_) | Source::Timer(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                    };
                    u16::from_be_bytes([upper_byte, lower_byte])
                }
            }
            Instruction::Index { args } => {
                match &args.op {
                    IndexOp::Assign => {
                        match &args.source {
                            IndexSource::Value(address) => 0xA000 | (u16::from(address) & 0x0FFF),
                            IndexSource::Register(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                        }
                    }
                    IndexOp::Add => {
                        match &args.source {
                            IndexSource::Value(_) => Err(Error::NoOpcodeError(instruction.clone()))?,
                            IndexSource::Register(vx) => 0xF01E | u16::from_be_bytes([vx.into(), 0]),
                        }
                    }
                }
            }
            Instruction::Arithmetic { args } => {
                match &args.target {
                    Target::Register(vx) => match &args.source {
                        Source::Byte(byte) => match &args.op {
                            BinaryOp::Assign =>
                                0x6000 | u16::from_be_bytes([vx.into(), byte.into()]),
                            BinaryOp::AddWrapping =>
                                0x7000 | u16::from_be_bytes([vx.into(), byte.into()]),
                            BinaryOp::Random =>
                                0xC000 | u16::from_be_bytes([vx.into(), byte.into()]),
                            _ => Err(Error::NoOpcodeError(instruction.clone()))?,
                        },
                        Source::Register(vy) => {
                            let lowest_nibble: u8 = match &args.op {
                                BinaryOp::Assign => 0x0,
                                BinaryOp::BitOr => 0x1,
                                BinaryOp::BitAnd => 0x2,
                                BinaryOp::BitXor => 0x3,
                                BinaryOp::Add => 0x4,
                                BinaryOp::Subtract => 0x5,
                                BinaryOp::BitShiftRight => 0x6,
                                BinaryOp::SubtractAlt => 0x7,
                                BinaryOp::BitShiftLeft => 0xE,
                                BinaryOp::AddWrapping | BinaryOp::Random => Err(Error::NoOpcodeError(instruction.clone()))?,
                            };
                            u16::from_be_bytes([u8::from(vx) | 0x80, u8::from(vy).rotate_left(4) | lowest_nibble])
                        }
                        Source::Timer(timer) => match timer {
                            Timer::Delay => 0xF007 | u16::from_be_bytes([u8::from(vx), 0]),
                            Timer::Sound => Err(Error::NoOpcodeError(instruction.clone()))?,
                        }
                    }
                    Target::Timer(timer) => {
                        if args.op != BinaryOp::Assign {
                            Err(Error::NoOpcodeError(instruction.clone()))
                        } else {
                            let lower_byte: u8 = match timer {
                                Timer::Delay => 0x15,
                                Timer::Sound => 0x18,
                            };
                            let upper_byte: u8 = match &args.source {
                                // todo: replace panics with this elsewhere
                                Source::Register(vx) => 0xF0u8 | u8::from(vx),
                                Source::Byte(_) | Source::Timer(_) =>
                                    Err(Error::NoOpcodeError(instruction.clone()))?,
                            };
                            Ok(u16::from_be_bytes([upper_byte, lower_byte]))
                        }?
                    }
                }
            }
            // todo: deduplicate
            Instruction::Font { args } => 0xF029 | u16::from_be_bytes([u8::from(&args.register), 0]),
            Instruction::KeyAwait { args } => 0xF00A | u16::from_be_bytes([u8::from(&args.register), 0]),
            Instruction::BinaryCodedDecimal { args } => 0xF033 | u16::from_be_bytes([u8::from(&args.register), 0]),
        };
        Ok(OpCode(op_code.into()))
    }
}

impl TryFrom<OpCode> for Instruction {
    type Error = Error;

    fn try_from(opcode: OpCode) -> Result<Self> {
        let (highest, rest) = (opcode.0.0 & 0xF000, opcode.0.0 & 0x0FFF);
        match highest >> 12 {
            0 => match rest {
                0x0E0 => Ok(Instruction::Graphics(Graphics::Clear)),
                0x0EE => Ok(Instruction::Flow(Flow::Return)),
                0x0F0 => Ok(Instruction::Exit),
                // todo: NullOpcode() instead? (because 0x0000 is likely to be due to PC pointing to uninitialized memory)
                0x0000 => Err(Error::InvalidOpCode(opcode)),
                rest => {
                    let args = JumpArgs { address: rest.try_into()?, register: None };
                    Ok(Instruction::Flow(Flow::Sys { args }))
                }
            },
            highest @ (0x1 | 0x2 | 0xA | 0xB) => {
                let register = if highest == 0xB { Some(Register::try_from(0).unwrap()) } else { None };
                let args = JumpArgs { address: rest.try_into()?, register };
                Ok(match highest {
                    0x1 | 0xB => Instruction::Flow(Flow::Jump { args }),
                    0x2 => Instruction::Flow(Flow::Call { args }),
                    0xA => Instruction::Index { args: IndexOpArgs::assign(args.address) },
                    _ => panic!("how did we get here?!"),
                })
            }
            highest @ (0x3 | 0x4 | 0x5 | 0x9) => {
                let [register, lower_byte] = rest.to_be_bytes();
                let args = BranchArgs {
                    lhs: Source::Register(register.try_into()?),
                    rhs: if highest == 0x3 || highest == 0x4 {
                        Source::Byte(lower_byte.into())
                    } else {
                        if 0xF & lower_byte == 0 {
                            Ok(Source::Register(Register::try_from(lower_byte.rotate_right(4) & 0xF)?))
                        } else {
                            Err(Error::InvalidOpCode(opcode))
                        }?
                    },
                    comparator: if highest == 0x3 || highest == 0x5 {
                        Comparator::Equal
                    } else {
                        Comparator::NotEqual
                    },
                };
                Ok(Instruction::Flow(Flow::Branch { args }))
            }
            highest @ (0x6 | 0x7 | 0xC) => {
                let [register, lower_byte] = rest.to_be_bytes();
                let args = BinaryOpArgs {
                    source: Source::Byte(lower_byte.into()),
                    target: Target::Register(register.try_into()?),
                    op: match highest {
                        0x6 => Ok(BinaryOp::Assign),
                        0x7 => Ok(BinaryOp::AddWrapping),
                        0xC => Ok(BinaryOp::Random),
                        _ => Err(Error::InvalidOpCode(opcode))
                    }?,
                };
                Ok(Instruction::Arithmetic { args })
            }
            0x8 => {
                let lowest = rest & 0x00F;
                let [vx, vy] = rest.to_be_bytes();
                let args = BinaryOpArgs {
                    source: Source::Register((vy & 0xF0).rotate_right(4).try_into()?),
                    target: Target::Register(vx.try_into()?),
                    op: match lowest {
                        0x0 => BinaryOp::Assign,
                        0x1 => BinaryOp::BitOr,
                        0x2 => BinaryOp::BitAnd,
                        0x3 => BinaryOp::BitXor,
                        0x4 => BinaryOp::Add,
                        0x5 => BinaryOp::Subtract,
                        0x6 => BinaryOp::BitShiftRight,
                        0x7 => BinaryOp::SubtractAlt,
                        0xE => BinaryOp::BitShiftLeft,
                        _ => Err(Error::InvalidOpCode(opcode))?,
                    },
                };
                Ok(Instruction::Arithmetic { args })
            }
            0xD => {
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
            0xF => {
                let register = Register::try_from((rest & 0x0F00).to_be_bytes()[0])?;
                match rest & 0x00FF {
                    0x0A => Ok(Instruction::KeyAwait { args: RegisterArgs { register } }),
                    0x07 => Ok(Instruction::Arithmetic {
                        args: BinaryOpArgs {
                            op: BinaryOp::Assign,
                            source: Source::Timer(Timer::Delay),
                            target: Target::Register(register),
                        }
                    }),
                    0x1E => Ok(Instruction::Index { args: IndexOpArgs::add(register) }),
                    0x29 => Ok(Instruction::Font { args: RegisterArgs { register } }),
                    0x33 => Ok(Instruction::BinaryCodedDecimal { args: RegisterArgs { register } }),
                    byte @ (0x15 | 0x18) => {
                        let target = Target::Timer(if byte == 0x15 { Timer::Delay } else { Timer::Sound });
                        let source = Source::Register(register);
                        // todo: different args for this? presence of carry flag here is confusing
                        Ok(Instruction::Arithmetic { args: BinaryOpArgs { target, source, op: BinaryOp::Assign } })
                    }
                    _ => Err(Error::InvalidOpCode(opcode)),
                }
            }
            _ => Err(Error::InvalidOpCode(opcode)),
        }
    }
}