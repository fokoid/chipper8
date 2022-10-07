use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use thiserror::Error;

use crate::tokens::{Token, Tokens};

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse int error")]
    IoError(#[from] ParseIntError),
    #[error("meta command syntax error")]
    MetaSyntaxError(String),
    #[error("syntax error")]
    SyntaxError(String),
    #[error("opcode syntax error")]
    OpCodeSyntaxError(String),
    #[error("invalid opcode error")]
    InvalidOpCode(OpCode),
}

type Result<T> = std::result::Result<T, Error>;

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
}

impl MetaCommand {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Meta(s)) => Err(Error::MetaSyntaxError(format!("invalid meta command '{}'", s))),
            s => Err(Error::MetaSyntaxError(format!("expected meta command token but found '{:?}'", s))),
        }
    }
}

impl Display for MetaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => todo!("add some metacommands"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    ClearScreen,
}

impl Instruction {
    pub fn parse(mut tokens: Tokens) -> Result<Instruction> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Other("cls")) => Ok(Self::ClearScreen),
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}

impl From<&Instruction> for OpCode {
    fn from(instruction: &Instruction) -> Self {
        OpCode(
            match instruction {
                Instruction::ClearScreen => 0x00E0,
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
            _ => Err(Error::InvalidOpCode(OpCode(self.0))),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClearScreen => write!(f, "cls"),
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