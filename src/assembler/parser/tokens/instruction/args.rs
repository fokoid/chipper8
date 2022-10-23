use crate::{Error, Result};
use crate::machine::instruction::args::{DrawArgs, RegisterArgs, SetArgs, Source, Target, Timer};
use crate::machine::instruction::SetAddressArgs;
use crate::machine::types::{Address, Nibble, Register};

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for SetAddressArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        let address = Address::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("expected an address"))
        )?)?;
        Ok(Self { address })
    }
}

impl TryFrom<Tokens<'_>> for SetArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        let target = Target::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("set requires a target"))
        )?)?;
        let source = Source::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("set requires a source"))
        )?)?;
        // carry flag only matters for arithmetic, not assignment. the only case when carry flag is
        // not set is constant addition 0x7000
        let carry = if let Source::Byte(_) = &source { false } else { true };
        Ok(Self { target, source, carry })
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
            token @ (Token::Hex(_) | Token::Other(_)) => {
                Ok(Self::Byte(token.try_into()?))
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
        let height = Nibble::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("draw requires a height"))
        )?)?;
        Ok(Self { x, y, height })
    }
}

impl TryFrom<Tokens<'_>> for RegisterArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        let register = Register::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("expected a register"))
        )?)?;
        Ok(Self { register })
    }
}