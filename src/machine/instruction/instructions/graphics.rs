use std::fmt::Debug;

use ux::u4;

use crate::{Error, Result};
use crate::command::tokens::{Token, Tokens};

use super::Register;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawArgs {
    pub x: Register,
    pub y: Register,
    pub height: u4,
}

impl DrawArgs {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        let x = Register::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("draw requires a register"))
        )?)?;
        let y = Register::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("draw requires a second register"))
        )?)?;
        let height = u8::from_str_radix(match tokens.next().ok_or(
            Error::SyntaxError(String::from("draw requires a height"))
        )? {
            Token::Other(s) => Ok(s),
            x => Err(Error::SyntaxError(format!("expected value, got {:?}", x)))
        }?, 16)?;
        Ok(Self { x, y, height: height.try_into().map_err(|_error|
            Error::IntSizeError(String::from("nibble"), height.into())
        )? })
    }
}