use std::fmt::{Debug, Display, Formatter};

pub use instructions::{Register, DrawArgs, SetArgs, Source, Target, Timer};
pub use opcode::OpCode;

mod opcode;
mod instructions;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Exit,
    ClearScreen,
    Jump(u16),
    Set { args: SetArgs },
    Add(u8, u8),
    IndexSet(u16),
    TimerGet(u8),
    Draw{ args: DrawArgs },
    Font(u8),
    AwaitKey(u8),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "exit"),
            Self::ClearScreen => write!(f, "cls"),
            Self::Jump(address) => write!(f, "jmp {:03X}", address),
            Self::Set { args } => write!(f, "set {} {}", args.target, args.source),
            Self::Add(register, value) => write!(f, "add {:01X} {:02X}", register, value),
            Self::IndexSet(value) => write!(f, "index set {:03X}", value),
            Self::Draw { args } => write!(f, "draw {} {} {:01X}", args.x, args.y, args.height),
            Self::Font(vx) => write!(f, "font {:01X}", vx),
            Self::TimerGet(register) => write!(f, "timer get {:02X}", register),
            Self::AwaitKey(register) => write!(f, "key await {:01X}", register),
        }
    }
}

#[cfg(test)]
mod tests;