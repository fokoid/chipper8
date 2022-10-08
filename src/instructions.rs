use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use thiserror::Error;

use crate::tokens::{Token, Tokens};

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),
    #[error("meta command syntax error")]
    MetaSyntaxError(String),
    #[error("syntax error")]
    SyntaxError(String),
    #[error("opcode syntax error")]
    OpCodeSyntaxError(String),
    #[error("invalid opcode error")]
    InvalidOpCode(OpCode),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Meta(MetaCommand),
    Instruction(Instruction),
}

impl Command {
    pub fn parse(mut tokens: Tokens) -> Result<Option<Self>> {
        match tokens.peek() {
            None | Some(Token::None) => Ok(None),
            Some(Token::Hex(_)) => {
                let opcode = &OpCode::parse(tokens)?;
                Ok(Some(Self::Instruction(opcode.as_instruction()?)))
            },
            Some(Token::Meta(_)) => Ok(Some(Self::Meta(MetaCommand::parse(tokens)?))),
            Some(Token::Other(_)) => Ok(Some(Self::Instruction(Instruction::parse(tokens)?))),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MetaCommand {
    Reset,
    Load(String, u16),
    Step,
    Play,
    Pause,
}

impl MetaCommand {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Meta(".reset")) => Ok(MetaCommand::Reset),
            Some(Token::Meta(".load")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    let path = String::from(s);
                    match tokens.next() {
                        Some(Token::Other(s)) => Ok(
                            MetaCommand::Load(path, u16::from_str_radix(s, 16)?)
                        ),
                        Some(x) => Err(Error::MetaSyntaxError(format!(".load requires an address but got {:?}", x))),
                        None => Err(Error::MetaSyntaxError(format!(".load requires an address"))),
                    }
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(".load requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(".load requires a path"))),
            }
            Some(Token::Meta(".step")) => Ok(MetaCommand::Step),
            Some(Token::Meta(".play")) => Ok(MetaCommand::Play),
            Some(Token::Meta(".pause")) => Ok(MetaCommand::Pause),
            Some(Token::Meta(s)) => Err(Error::MetaSyntaxError(format!("invalid meta command '{}'", s))),
            s => Err(Error::MetaSyntaxError(format!("expected meta command token but found '{:?}'", s))),
        }
    }
}

impl Display for MetaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset => write!(f, ".reset"),
            Self::Load(path, address) => write!(f, ".load {} {:03X}", path, address),
            Self::Step => write!(f, ".step"),
            Self::Play => write!(f, ".play"),
            Self::Pause => write!(f, ".pause"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    Set(u8, u8),
    Add(u8, u8),
    IndexSet(u16),
    TimerSound(u8),
    Draw(u8, u8, u8),
    Font(u8),
}

impl Instruction {
    pub fn parse(mut tokens: Tokens) -> Result<Instruction> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Other("cls")) => Ok(Self::ClearScreen),
            Some(Token::Other("jmp")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Jump(
                    // todo: bounds checking (12 bit address)
                    u16::from_str_radix(s, 16)?
                )),
                Some(x) => Err(Error::SyntaxError(format!("jmp requires an address but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("jmp requires an address"))),
            }
            Some(Token::Other("set")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    // todo: bounds checking (12 bit address)
                    let register = u8::from_str_radix(s, 16)?;
                    match tokens.next() {
                        Some(Token::Other(s)) => Ok(Instruction::Set(
                            register,
                            u8::from_str_radix(s, 16)?,
                        )),
                        Some(x) => Err(Error::SyntaxError(format!("set requires a value but got {:?}", x))),
                        None => Err(Error::SyntaxError(format!("set requires a value"))),
                    }
                },
                Some(x) => Err(Error::SyntaxError(format!("set requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("set requires a register"))),
            },
            Some(Token::Other("add")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    // todo: bounds checking (12 bit address)
                    let register = u8::from_str_radix(s, 16)?;
                    match tokens.next() {
                        Some(Token::Other(s)) => Ok(Instruction::Add(
                            register,
                            u8::from_str_radix(s, 16)?,
                        )),
                        Some(x) => Err(Error::SyntaxError(format!("add requires a value but got {:?}", x))),
                        None => Err(Error::SyntaxError(format!("add requires a value"))),
                    }
                },
                Some(x) => Err(Error::SyntaxError(format!("addset requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("addset requires a register"))),
            },
            Some(Token::Other("index")) => match tokens.next() {
                Some(Token::Other("set")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::IndexSet(
                        // todo: bounds checking (12 bit address)
                        u16::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("jmp requires an address but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("jmp requires an address"))),
                },
                None | Some(_) => Err(Error::SyntaxError(String::from("allowed index sub commands: set"))),
            },
            Some(Token::Other("draw")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    // todo: bounds checking (12 bit address)
                    let vx = u8::from_str_radix(s, 16)?;
                    match tokens.next() {
                        Some(Token::Other(s)) => {
                            let vy = u8::from_str_radix(s, 16)?;
                            match tokens.next() {
                                Some(Token::Other(s)) => Ok(Instruction::Draw(
                                    vx, vy, u8::from_str_radix(s, 16)?,
                                )),
                                Some(x) => Err(Error::SyntaxError(format!("draw requires a value {:?}", x))),
                                None => Err(Error::SyntaxError(format!("draw requires a value"))),
                            }
                        },
                        Some(x) => Err(Error::SyntaxError(format!("draw requires a second register but got {:?}", x))),
                        None => Err(Error::SyntaxError(format!("draw requires a second register"))),
                    }
                },
                Some(x) => Err(Error::SyntaxError(format!("draw requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("draw requires a register"))),
            },
            Some(Token::Other("timer")) => match tokens.next() {
                Some(Token::Other("sound")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::TimerSound(
                        u8::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("timer sound requires a value but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("timer sound requires a value"))),
                },
                Some(_) => Err(Error::SyntaxError(String::from("allowed timer sub commands: sound"))),
                None => Err(Error::SyntaxError(String::from("timer requires a sub command"))),
            },
            Some(Token::Other("font")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Font(u8::from_str_radix(s, 16)?)),
                Some(x) => Err(Error::SyntaxError(format!("font requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("font requires a register"))),
            },
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}

impl From<&Instruction> for OpCode {
    fn from(instruction: &Instruction) -> Self {
        OpCode(
            match instruction {
                Instruction::ClearScreen => 0x00E0,
                Instruction::Jump(address) => 0x1000 | (address & 0x0FFF),
                Instruction::Set(register, value) =>
                    0x6000 | u16::from_be_bytes([*register, *value]),
                Instruction::Add(register, value) =>
                    0x7000 | u16::from_be_bytes([*register, *value]),
                Instruction::IndexSet(value) => 0xA000 | (value & 0x0FFF),
                Instruction::Draw(vx, vy, height) =>
                    0xD000 | u16::from_be_bytes([*vx, vy.rotate_left(4) | *height]),
                Instruction::TimerSound(value) => 0xF018 | u16::from_be_bytes([*value, 0]),
                Instruction::Font(register) => 0xF029 | u16::from_be_bytes([*register, 0]),
            }
        )
    }
}

impl OpCode {
    pub fn as_instruction(&self) -> Result<Instruction> {
        match self.0 & 0xF000 {
            0 => match self.0 & 0x0FFF {
                0x0E0 => Ok(Instruction::ClearScreen),
                _ => Err(Error::InvalidOpCode(OpCode(self.0))),
            },
            0x1000 => Ok(Instruction::Jump(self.0 & 0x0FFF)),
            0x6000 => {
                let [register, value] = (self.0 & 0x0FFF).to_be_bytes();
                Ok(Instruction::Set(register, value))
            },
            0x7000 => {
                let [register, value] = (self.0 & 0x0FFF).to_be_bytes();
                Ok(Instruction::Add(register, value))
            },
            0xA000 => Ok(Instruction::IndexSet(self.0 & 0x0FFF)),
            0xD000 => {
                let [vx, lower] = (self.0 & 0xFFF).to_be_bytes();
                let vy = lower.rotate_left(4) & 0x0F;
                let height = lower & 0x0F;
                Ok(Instruction::Draw(vx, vy, height))
            }
            0xF000 => {
                match self.0 & 0x00FF {
                    0x18 => Ok(Instruction::TimerSound((self.0 & 0x0F00).to_be_bytes()[0])),
                    0x29 => Ok(Instruction::Font((self.0 & 0x0F00).to_be_bytes()[0])),
                    _ => Err(Error::InvalidOpCode(OpCode(self.0))),
                }
            }
            _ => Err(Error::InvalidOpCode(OpCode(self.0))),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClearScreen => write!(f, "cls"),
            Self::Jump(address) => write!(f, "jmp {:03X}", address),
            Self::Set(register, value) => write!(f, "set {:01X} {:02X}", register, value),
            Self::Add(register, value) => write!(f, "add {:01X} {:02X}", register, value),
            Self::IndexSet(value) => write!(f, "index set {:03X}", value),
            Self::Draw(vx, vy, height) => write!(f, "draw {:01X} {:01X} {:01X}", vx, vy, height),
            Self::Font(vx) => write!(f, "font {:01X}", vx),
            Self::TimerSound(value) => write!(f, "timer sound {:02X}", value),
        }
    }
}

pub struct OpCode(pub u16);

impl OpCode {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        match tokens.next() {
            Some(Token::Hex(s)) => {
                Ok(OpCode(u16::from_str_radix(&s[2..], 16)?))
            },
            x => Err(Error::OpCodeSyntaxError(format!("{:?}", x))),
        }
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

impl Command {
    pub fn opcode(&self) -> Option<OpCode> {
        match self {
            Self::Meta(_) => None,
            Self::Instruction(instruction) => Some(instruction.into()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Meta(meta) => write!(f, "{}", meta),
            Self::Instruction(instruction) => write!(f, "{}", instruction),
        }
    }
}