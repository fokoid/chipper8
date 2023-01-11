use crate::{Error, Result};
use crate::machine::instruction::{Flow, Graphics, Instruction, OpCode};

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for Instruction {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        // special handling for arithmetic instructions since the first token should be forwarded to
        // arg parsing rather than being consumed
        if let Some(Token::Register(vx)) = tokens.peek() {
            // index arithmetic treated separately since RHS values are 12 bit not 8 bit
            if *vx != "I" {
                return Ok(Instruction::Arithmetic { args: tokens.try_into()? });
            }
        }
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Register("I")) => Ok(Instruction::Index { args: tokens.try_into()? }),
            Some(Token::Other("exit")) => Ok(Self::Exit),
            Some(Token::Other("graphics")) => Ok(Self::Graphics(tokens.try_into()?)),
            Some(Token::Other("return")) => Ok(Instruction::Flow(Flow::Return)),
            Some(Token::Other("sys")) => Ok(Instruction::Flow(Flow::Sys { args: tokens.try_into()? })),
            Some(Token::Other("jump")) => Ok(Instruction::Flow(Flow::Jump { args: tokens.try_into()? })),
            Some(Token::Other("call")) => Ok(Instruction::Flow(Flow::Call { args: tokens.try_into()? })),
            Some(Token::Other("branch")) => Ok(Instruction::Flow(Flow::Branch { args: tokens.try_into()? })),
            Some(Token::Other("font")) => Ok(Instruction::Font { args: tokens.try_into()? }),
            Some(Token::Other("key")) => match tokens.next() {
                Some(Token::Other("await")) => Ok(Instruction::KeyAwait { args: tokens.try_into()? }),
                Some(x) => Err(Error::SyntaxError(format!("key requires a subcommand, but got {:?}; allowed: await", x))),
                None => Err(Error::SyntaxError(format!("key requires a subcommand; allowed: await"))),
            }
            Some(Token::Other("bcd")) => Ok(Instruction::BinaryCodedDecimal { args: tokens.try_into()? }),
            Some(token @ Token::Hex(_)) => {
                let opcode = OpCode::try_from(token)?;
                Ok(opcode.try_into()?)
            }
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}

impl TryFrom<Tokens<'_>> for Graphics {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        match tokens.next() {
            Some(Token::Other("clear")) => Ok(Graphics::Clear),
            Some(Token::Other("draw")) => Ok(Graphics::Draw { args: tokens.try_into()? }),
            Some(Token::Other(s)) => Err(Error::SyntaxError(format!(
                "unrecognized graphics system instruction {}", s
            ))),
            Some(x) => Err(Error::SyntaxError(format!(
                "expected graphics system instruction, got {:?}", x
            ))),
            None => Err(Error::SyntaxError(format!("expected graphics system instruction"))),
        }
    }
}