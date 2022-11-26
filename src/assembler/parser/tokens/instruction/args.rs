use crate::{Error, Result};
use crate::machine::instruction::args::{BinaryOp, BinaryOpArgs, BranchArgs, Comparator, DrawArgs, JumpArgs, RegisterArgs, Source, Target, Timer};
use crate::machine::types::{Address, Nibble, Register};

use super::{Token, Tokens};

impl TryFrom<Tokens<'_>> for JumpArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        let address = Address::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("expected an address"))
        )?)?;
        let register = if let Some(token) = tokens.next() {
            Some(token.try_into()?)
        } else { None };
        Ok(Self { address, register })
    }
}

impl TryFrom<Tokens<'_>> for BinaryOpArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        let target = Target::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("arithmetic requires a target"))
        )?)?;
        let op = BinaryOp::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("arithmetic requires an operation"))
        )?)?;
        let source = Source::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("arithmetic requires a source"))
        )?)?;
        Ok(Self { target, source, op })
    }
}

impl TryFrom<Token<'_>> for Target {
    type Error = Error;

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        Ok(match token {
            Token::Register("T") => Self::Timer(Timer::Delay),
            Token::Register("S") => Self::Timer(Timer::Sound),
            Token::Register(_) => Self::Register(token.try_into()?),
            x =>
                Err(Error::SyntaxError(format!("expected target, found {:?}", x)))?,
        })
    }
}

impl TryFrom<Token<'_>> for Source {
    type Error = Error;

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        Ok(match token {
            Token::Register("T") => Self::Timer(Timer::Delay),
            Token::Register("S") => Self::Timer(Timer::Sound),
            Token::Register(_) => Self::Register(token.try_into()?),
            token @ (Token::Hex(_) | Token::Other(_)) => Self::Byte(token.try_into()?),
            x => Err(Error::SyntaxError(format!("expected register or value, found {:?}", x)))?,
        })
    }
}

impl TryFrom<Token<'_>> for BinaryOp {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        match token {
            Token::Other("=") => Ok(BinaryOp::Assign),
            Token::Other("+=") => Ok(BinaryOp::Add),
            Token::Other("+~") => Ok(BinaryOp::AddWrapping),
            x => Err(Error::SyntaxError(format!("expected binary operation, found {:?}", x))),
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

impl TryFrom<Option<Token<'_>>> for Comparator {
    type Error = Error;

    fn try_from(token: Option<Token<'_>>) -> Result<Self> {
        match token {
            Some(Token::Other(s)) if s == "!" => Ok(Self::NotEqual),
            Some(x) => Err(Error::SyntaxError(format!("expected a comparator, got {:?}", x))),
            None => Ok(Self::Equal)
        }
    }
}

impl TryFrom<Tokens<'_>> for BranchArgs {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        let lhs = Source::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("conditional requires a LHS expression"))
        )?)?;
        let rhs = Source::try_from(tokens.next().ok_or(
            Error::SyntaxError(String::from("conditional requires a LHS expression"))
        )?)?;
        let comparator = Comparator::try_from(tokens.next())?;
        Ok(Self { lhs, rhs, comparator })
    }
}