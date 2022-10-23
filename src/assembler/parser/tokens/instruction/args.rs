use crate::{Error, Result};
use crate::machine::instruction::args::{DrawArgs, SetArgs, Source, Target, Timer};
use crate::machine::types::Register;

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for SetArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        let target = Target::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("set requires a target"))
        )?)?;
        let source = Source::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("set requires a source"))
        )?)?;
        if let Some(x) = tokens.next() {
            Err(Error::SyntaxError(format!("unexpected token {:?}", x)))
        } else {
            Ok(Self { target, source })
        }
    }
}

impl TryFrom<Token<'_>> for Target {
    type Error = Error;

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        match token {
            Token::Register(_) => Ok(Self::Register(token.try_into()?)),
            Token::Other("delay") => Ok(Self::Timer(Timer::Delay)),
            Token::Other("sound") => Ok(Self::Timer(Timer::Sound)),
            x => {
                Err(Error::SyntaxError(format!("expected target, found {:?}", x)))
            }
        }
    }
}

impl TryFrom<Token<'_>> for Source {
    type Error = Error;

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        match token {
            Token::Register(_) => {
                Ok(Self::Register(token.try_into()?))
            }
            Token::Other(s) => {
                Ok(Self::Value(u8::from_str_radix(s, 16)?))
            }
            x => {
                Err(Error::SyntaxError(format!("expected register or value, found {:?}", x)))
            }
        }
    }
}

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
        Ok(Self {
            x,
            y,
            height: height.try_into().map_err(|_error|
                Error::IntSizeError(String::from("nibble"), height.into())
            )?,
        })
    }
}