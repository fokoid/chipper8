use ux::{u12, u4};

use crate::{Error, Result};
use crate::machine::types::{Address, Byte, Nibble, Register};

#[derive(Debug)]
pub enum Token<'a> {
    None,
    Meta(&'a str),
    Hex(&'a str),
    Register(&'a str),
    Other(&'a str),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(s: &'a str) -> Self {
        match s.chars().nth(0) {
            None => Self::None,
            Some('0') => match s.chars().nth(1) {
                Some('x') => Self::Hex(&s[2..]),
                _ => Self::Other(s),
            }
            Some('V') if s.len() == 2 => Self::Register(&s[1..]),
            Some(':') => Self::Meta(s),
            Some(_) => Self::Other(s),
        }
    }
}

impl<'a> From<Token<'a>> for &'a str {
    fn from(token: Token<'a>) -> Self {
        match token {
            Token::None => "",
            Token::Hex(s) | Token::Meta(s) | Token::Other(s) | Token::Register(s) => s,
        }
    }
}

impl<'a> From<Token<'a>> for String {
    fn from(token: Token<'a>) -> Self {
        let s: &'a str = token.into();
        String::from(s)
    }
}

impl TryFrom<Token<'_>> for u4 {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        let value = u8::try_from(token)?;
        value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("nibble"), value.into())
        )
    }
}

impl TryFrom<Token<'_>> for u8 {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        match token {
            Token::Other(s) => {
                Ok(u8::from_str_radix(s, 10)?)
            }
            Token::Hex(s) => {
                Ok(u8::from_str_radix(s, 16)?)
            }
            x => Err(Error::SyntaxError(format!("expected decimal or hex value, found {:?}", x))),
        }
    }
}

impl TryFrom<Token<'_>> for u12 {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        let value = u16::try_from(token)?;
        value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("word"), value.into())
        )
    }
}

impl TryFrom<Token<'_>> for u16 {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        match token {
            Token::Other(s) => {
                Ok(u16::from_str_radix(s, 10)?)
            }
            Token::Hex(s) => {
                Ok(u16::from_str_radix(s, 16)?)
            }
            x => Err(Error::SyntaxError(format!("expected decimal or hex value, found {:?}", x))),
        }
    }
}

impl TryFrom<Token<'_>> for Register {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Register(s) => {
                let value = u8::from_str_radix(s, 16)?;
                value.try_into()
            }
            x => Err(Error::SyntaxError(format!("expected register, found {:?}", x))),
        }
    }
}

impl TryFrom<Token<'_>> for Nibble {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        Ok(Self(u4::try_from(token)?))
    }
}

impl TryFrom<Token<'_>> for Byte {
    type Error = Error;

    fn try_from(token: Token<'_>) -> Result<Self> {
        Ok(Self(u8::try_from(token)?))
    }
}

impl TryFrom<Token<'_>> for Address {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        Ok(Self(u12::try_from(token)?))
    }
}