use std::fmt::{Debug, Display, Formatter};

use super::args::{DrawArgs, SetAddressArgs, SetArgs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    ClearScreen,
    Jump { args: SetAddressArgs },
    IndexSet { args: SetAddressArgs },
    Set { args: SetArgs },
    Add { args: SetArgs },
    TimerGet(u8),
    Draw { args: DrawArgs },
    Font(u8),
    AwaitKey(u8),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::ClearScreen => write!(f, "cls"),
            Self::Jump { args } => write!(f, "jmp {}", args.address),
            Self::IndexSet { args } => write!(f, "index {}", args.address),
            Self::Set { args } => write!(f, "set {} {}", args.target, args.source),
            Self::Add { args } => write!(f, "add {} {}", args.target, args.source),
            Self::Draw { args } => write!(f, "draw {} {} {}", args.x, args.y, args.height),
            Self::Font(vx) => write!(f, "font {:01X}", vx),
            Self::TimerGet(register) => write!(f, "timer get {:02X}", register),
            Self::AwaitKey(register) => write!(f, "key await {:01X}", register),
        }
    }
}

