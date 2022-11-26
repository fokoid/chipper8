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
            Self::Delay => write!(f, "VT"),
            Self::Sound => write!(f, "VS"),
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
    Timer(Timer),
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(x) => write!(f, "{}", x),
            Self::Register(vx) => write!(f, "{}", vx),
            Self::Timer(timer) => write!(f, "{}", timer),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Assign,
    Add,
    AddWrapping,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Assign => "=",
            Self::Add => "+=",
            // todo: nicer syntax
            Self::AddWrapping => "+~",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryOpArgs {
    pub target: Target,
    pub source: Source,
    pub op: BinaryOp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawArgs {
    pub x: Register,
    pub y: Register,
    pub height: Nibble,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisterArgs {
    pub register: Register,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JumpArgs {
    pub address: Address,
    pub register: Option<Register>,
}

impl Display for JumpArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(register) = &self.register {
            write!(f, "{} {}", self.address, register)
        } else {
            write!(f, "{}", self.address)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Comparator {
    Equal,
    NotEqual,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BranchArgs {
    pub lhs: Source,
    pub rhs: Source,
    pub comparator: Comparator,
}

impl Display for BranchArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", self.lhs, self.rhs, match self.comparator {
            Comparator::Equal => "",
            Comparator::NotEqual => " !",
        })
    }
}