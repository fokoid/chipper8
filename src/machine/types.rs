use std::fmt::{Display, Formatter};

use ux::{u12, u4};

use crate::{Error, Result};

pub type Timer = u8;
pub type Pointer = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nibble(pub u4);

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#01X}", self.0)
    }
}

impl From<u4> for Nibble {
    fn from(value: u4) -> Self { Self(value) }
}

impl TryFrom<u8> for Nibble {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let nibble: u4 = value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("4 bit nibble"), value.into())
        )?;
        Ok(Self(nibble))
    }
}

impl From<&Nibble> for u8 {
    fn from(nibble: &Nibble) -> Self {
        nibble.0.into()
    }
}

impl From<&Nibble> for usize {
    fn from(nibble: &Nibble) -> Self {
        u8::from(nibble) as usize
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Byte(pub u8);

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#03X}", self.0)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self { Self(value) }
}

impl From<&Byte> for u8 {
    fn from(byte: &Byte) -> Self {
        byte.0
    }
}

impl From<&Byte> for usize {
    fn from(byte: &Byte) -> Self {
        u8::from(byte) as usize
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Address(pub u12);

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#05X}", self.0)
    }
}

impl TryFrom<u16> for Address {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        let address: u12 = value.try_into().map_err(|_error|
            Error::IntSizeError(String::from("12 bit address"), value.into())
        )?;
        Ok(Self(address))
    }
}

impl From<&Address> for u16 {
    fn from(address: &Address) -> Self {
        u16::from(address.0)
    }
}

impl From<&Address> for usize {
    fn from(address: &Address) -> Self {
        u16::from(address) as usize
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Word(pub u16);

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self { Self(value) }
}

impl From<&Word> for u16 {
    fn from(word: &Word) -> Self {
        word.0
    }
}

impl From<&Word> for usize {
    fn from(word: &Word) -> Self {
        u16::from(word) as usize
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Register(pub Nibble);

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{:01X}", self.0.0)
    }
}

impl TryFrom<u8> for Register {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(Self(Nibble::try_from(value)?))
    }
}

impl From<&Register> for u8 {
    fn from(register: &Register) -> Self {
        u8::from(&register.0)
    }
}

impl From<&Register> for usize {
    fn from(register: &Register) -> Self {
        u8::from(register) as usize
    }
}