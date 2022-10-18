use std::fmt::{Display, Formatter};

use chipper8::machine::Pointer;

pub struct Decimal(pub u16);

pub struct Nibble(pub u8);

pub struct Byte(pub u8);

pub struct Address(pub u16);

pub struct Word(pub u16);

pub struct Register(pub u8);

impl Into<String> for Decimal {
    fn into(self) -> String {
        format!("{:>4}", self.0)
    }
}

impl Into<String> for Byte {
    fn into(self) -> String {
        format!("{:02X}", self.0)
    }
}

impl Into<String> for Address {
    fn into(self) -> String {
        format!("{:03X}", self.0)
    }
}

impl Into<String> for Word {
    fn into(self) -> String {
        format!("{:04X}", self.0)
    }
}

impl From<Pointer> for Address {
    fn from(pointer: Pointer) -> Self {
        Self(pointer as u16)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<u8> for Decimal {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl From<Pointer> for Decimal {
    fn from(pointer: Pointer) -> Self {
        Self(pointer as u16)
    }
}

impl From<u8> for Register {
    fn from(register: u8) -> Self {
        Self(register & 0xF as u8)
    }
}

impl From<usize> for Register {
    fn from(register: usize) -> Self {
        (register as u8).into()
    }
}

impl Into<String> for Register {
    fn into(self) -> String {
        format!("V{:01X}", self.0)
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<u8> for Nibble {
    fn from(register: u8) -> Self {
        Self(register & 0xF)
    }
}

impl From<usize> for Nibble {
    fn from(register: usize) -> Self {
        Self((register & 0xF) as u8)
    }
}

impl Into<String> for Nibble {
    fn into(self) -> String {
        format!("{}", &self)
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:01X}", self.0)
    }
}