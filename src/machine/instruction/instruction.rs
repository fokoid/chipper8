use std::fmt::{Debug, Display, Formatter};

use super::args::{BinaryOpArgs, BranchArgs, DrawArgs, JumpArgs, RegisterArgs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    Graphics(Graphics),
    Flow(Flow),
    IndexSet { args: JumpArgs },
    Arithmetic { args: BinaryOpArgs },
    Font { args: RegisterArgs },
    KeyAwait { args: RegisterArgs },
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::Graphics(graphics) => write!(f, "graphics {}", graphics),
            Self::Flow(flow) => write!(f, "{}", flow),
            Self::IndexSet { args } => write!(f, "index {}", args),
            Self::Arithmetic { args } => write!(f, "{} {} {}", args.target, args.op, args.source),
            Self::Font { args } => write!(f, "font {}", args.register),
            Self::KeyAwait { args } => write!(f, "key await {}", args.register),
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
