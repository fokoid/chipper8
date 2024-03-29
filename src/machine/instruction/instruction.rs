use std::fmt::{Debug, Display, Formatter};

use super::args::{BinaryOpArgs, BranchArgs, DrawArgs, IndexOpArgs, InputBranchArgs, JumpArgs, RegisterArgs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    Graphics(Graphics),
    Flow(Flow),
    Index { args: IndexOpArgs },
    Arithmetic { args: BinaryOpArgs },
    Input(Input),
    BinaryCodedDecimal { args: RegisterArgs },
    Memory(Memory),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Graphics {
    Clear,
    Draw { args: DrawArgs },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Flow {
    Return,
    Call { args: JumpArgs },
    Jump { args: JumpArgs },
    Sys { args: JumpArgs },
    Branch { args: BranchArgs },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Memory {
    Load { args: RegisterArgs },
    Save { args: RegisterArgs },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Await { args: RegisterArgs },
    Branch { args: InputBranchArgs },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::Graphics(graphics) => write!(f, "graphics {}", graphics),
            Self::Flow(flow) => write!(f, "{}", flow),
            Self::Index { args } => write!(f, "VI {} {}", args.op, args.source),
            Self::Arithmetic { args } => write!(f, "{} {} {}", args.target, args.op, args.source),
            Self::Input(input) => write!(f, "input {}", input),
            Self::Memory(memory) => write!(f, "mem {}", memory),
            Self::BinaryCodedDecimal { args } => write!(f, "bcd {}", args),
        }
    }
}

impl Display for Graphics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear => write!(f, "clear"),
            Self::Draw { args } => write!(f, "draw {} {} {}", args.x, args.y, args.height),
        }
    }
}

impl Display for Flow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sys { args } => write!(f, "sys {}", args),
            Self::Jump { args } => write!(f, "jump {}", args),
            Self::Call { args } => write!(f, "call {}", args),
            Self::Return => write!(f, "return"),
            Self::Branch { args } => write!(f, "branch {}", args),
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load { args } => write!(f, "load {}", args),
            Self::Save { args } => write!(f, "save {}", args),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Await { args } => write!(f, "await {}", args),
            Self::Branch { args } => write!(f, "branch {}", args),
        }
    }
}
