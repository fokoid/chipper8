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
            Some(Token::Other("jmp")) => Ok(Instruction::Jump { args: tokens.try_into()? }),
            Some(Token::Other("index")) => Ok(Instruction::IndexSet { args: tokens.try_into()? }),
            Some(Token::Other("set")) => Ok(Instruction::Set { args: tokens.try_into()? }),
            Some(Token::Other("add")) => Ok(Instruction::Add { args: tokens.try_into()? }),
            Some(Token::Other("draw")) => Ok(Instruction::Draw { args: tokens.try_into()? }),
            Some(Token::Other("get")) => match tokens.next() {
                Some(Token::Other("timer")) => Ok(Instruction::GetTimer { args: tokens.try_into()? }),
                Some(x) => Err(Error::SyntaxError(format!("get requires a subcommand, but got {:?}; allowed: timer", x))),
                None => Err(Error::SyntaxError(format!("get requires a subcommand; allowed: timer"))),
            }
            Some(Token::Other("font")) => Ok(Instruction::Font { args: tokens.try_into()? }),
            Some(Token::Other("key")) => match tokens.next() {
                Some(Token::Other("await")) => Ok(Instruction::KeyAwait { args: tokens.try_into()? }),
                Some(x) => Err(Error::SyntaxError(format!("key requires a subcommand, but got {:?}; allowed: await", x))),
                None => Err(Error::SyntaxError(format!("key requires a subcommand; allowed: await"))),
            }
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}
