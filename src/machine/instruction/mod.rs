use std::fmt::{Debug, Display, Formatter};

pub use instructions::{Register, SetArgs, Source, Target, Timer};
pub use opcode::OpCode;

use crate::{Error, Result};
use crate::command::tokens::{Token, Tokens};

mod opcode;
mod instructions;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    ClearScreen,
    Jump(u16),
    Set { args: SetArgs },
    Add(u8, u8),
    IndexSet(u16),
    TimerGet(u8),
    Draw(u8, u8, u8),
    Font(u8),
    AwaitKey(u8),
}

impl Instruction {
    pub fn parse(mut tokens: Tokens) -> Result<Instruction> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Other("exit")) => Ok(Self::Exit),
            Some(Token::Other("cls")) => Ok(Self::ClearScreen),
            Some(Token::Other("jmp")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Jump(
                    // todo: bounds checking (12 bit address)
                    u16::from_str_radix(s, 16)?
                )),
                Some(x) => Err(Error::SyntaxError(format!("jmp requires an address but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("jmp requires an address"))),
            }
            Some(Token::Other("set")) => Ok(Instruction::Set { args: SetArgs::parse(tokens)? }),
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
                }
                Some(x) => Err(Error::SyntaxError(format!("add requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("add requires a register"))),
            },
            Some(Token::Other("index")) => match tokens.next() {
                Some(Token::Other("set")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::IndexSet(
                        // todo: bounds checking (12 bit address)
                        u16::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("index set requires an address but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("index set requires an address"))),
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
                        }
                        Some(x) => Err(Error::SyntaxError(format!("draw requires a second register but got {:?}", x))),
                        None => Err(Error::SyntaxError(format!("draw requires a second register"))),
                    }
                }
                Some(x) => Err(Error::SyntaxError(format!("draw requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("draw requires a register"))),
            },
            Some(Token::Other("timer")) => match tokens.next() {
                Some(Token::Other("get")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::TimerGet(
                        u8::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("timer get requires a register but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("timer get requires a register"))),
                },
                Some(_) => Err(Error::SyntaxError(String::from("allowed timer sub commands: get"))),
                None => Err(Error::SyntaxError(String::from("timer requires a sub command"))),
            },
            Some(Token::Other("font")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Font(u8::from_str_radix(s, 16)?)),
                Some(x) => Err(Error::SyntaxError(format!("font requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("font requires a register"))),
            },
            Some(Token::Other("key")) => match tokens.next() {
                Some(Token::Other("await")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::AwaitKey(u8::from_str_radix(s, 16)?)),
                    Some(x) => Err(Error::SyntaxError(format!("key await requires a register but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("key await requires a register"))),
                }
                Some(x) => Err(Error::SyntaxError(format!("key requires a subcommand but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("key requires a subcommand"))),
            }
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::ClearScreen => write!(f, "cls"),
            Self::Jump(address) => write!(f, "jmp {:03X}", address),
            Self::Set { args } => write!(f, "set {} {}", args.target, args.source),
            Self::Add(register, value) => write!(f, "add {:01X} {:02X}", register, value),
            Self::IndexSet(value) => write!(f, "index set {:03X}", value),
            Self::Draw(vx, vy, height) => write!(f, "draw {:01X} {:01X} {:01X}", vx, vy, height),
            Self::Font(vx) => write!(f, "font {:01X}", vx),
            Self::TimerGet(register) => write!(f, "timer get {:02X}", register),
            Self::AwaitKey(register) => write!(f, "key await {:01X}", register),
        }
    }
}

#[cfg(test)]
mod tests;