use std::fmt::{Debug, Display, Formatter};

use super::args::{DrawArgs, RegisterArgs, SetAddressArgs, SetArgs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    Graphics(Graphics),
    Flow(Flow),
    IndexSet { args: SetAddressArgs },
    Set { args: SetArgs },
    Add { args: SetArgs },
    GetTimer { args: RegisterArgs },
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
    Jump { args: SetAddressArgs },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::Graphics(graphics) => write!(f, "graphics {}", graphics),
            Self::Flow(flow) => write!(f, "{}", flow),
            Self::IndexSet { args } => write!(f, "index {}", args.address),
            Self::Set { args } => write!(f, "set {} {}", args.target, args.source),
            Self::Add { args } => write!(f, "add {} {}", args.target, args.source),
            Self::Font { args } => write!(f, "font {}", args.register),
            Self::GetTimer { args } => write!(f, "get timer {}", args.register),
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
            Flow::Jump { args } => write!(f, "jump {}", args.address),
            Self::Return => write!(f, "return"),
        }
    }
}
