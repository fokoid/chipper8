use crate::{Error, Result};
use crate::parsing::{Token, Tokens};
use crate::machine::instruction::{DrawArgs, Register};

impl TryFrom<Tokens<'_>> for DrawArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
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
