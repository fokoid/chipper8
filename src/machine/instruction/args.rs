use std::fmt::{Debug, Display, Formatter};

use crate::machine::types::{Address, Byte, Nibble, Register};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source {
    Byte(Byte),
    Register(Register),
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(x) => write!(f, "{}", x),
            Self::Register(vx) => write!(f, "{}", vx),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetArgs {
    pub target: Target,
    pub source: Source,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAddressArgs {
    pub address: Address,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawArgs {
    pub x: Register,
    pub y: Register,
    pub height: Nibble,
}
