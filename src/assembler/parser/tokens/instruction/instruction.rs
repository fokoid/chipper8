use crate::{Error, Result};
use crate::machine::Instruction;

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for Instruction {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
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
            Some(Token::Other("set")) => Ok(Instruction::Set { args: tokens.try_into()? }),
            Some(Token::Other("draw")) => Ok(Instruction::Draw { args: tokens.try_into()? }),
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
