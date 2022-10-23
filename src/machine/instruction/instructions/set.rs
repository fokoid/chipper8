use std::fmt::{Debug, Display, Formatter};

use crate::{Error, Result};
use crate::command::tokens::{Token, Tokens};

use super::Register;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Timer {
    Delay,
    Sound,
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Delay => write!(f, "delay"),
            Self::Sound => write!(f, "sound"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Target {
    Register(Register),
    Timer(Timer),
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(vx) => write!(f, "{}", vx),
            Self::Timer(timer) => write!(f, "{}", timer),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source {
    Value(u8),
    Register(Register),
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(x) => write!(f, "{:02X}", x),
            Self::Register(vx) => write!(f, "{}", vx),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetArgs {
    pub target: Target,
    pub source: Source,
}

impl SetArgs {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
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