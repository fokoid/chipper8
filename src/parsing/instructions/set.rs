use crate::{Error, Result};
use crate::parsing::{Token, Tokens};
use crate::machine::instruction::{SetArgs, Target, Source, Timer};

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