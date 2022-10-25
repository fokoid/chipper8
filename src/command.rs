use std::fmt::{Debug, Display, Formatter};

use crate::machine::{Address, Instruction, OpCode};
use crate::ui;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Meta(MetaCommand),
    Instruction(Instruction),
}

impl Command {
    pub fn opcode(&self) -> Option<OpCode> {
        match self {
            Self::Meta(_) => None,
            Self::Instruction(instruction) => Some(instruction.try_into().ok()?),
        }
    }

    pub fn is_meta(&self) -> bool {
        match self {
            Self::Meta(_) => true,
            Self::Instruction(_) => false,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Meta(meta) => write!(f, "{}", meta),
            Self::Instruction(instruction) => write!(f, "{}", instruction),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MachineState {
    Demo,
}

impl Display for MachineState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Demo => write!(f, "{}", "demo"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetaCommand {
    Reset(Option<MachineState>),
    LoadRom(String, Option<Address>),
    DumpMachine(String),
    LoadMachine(String),
    UnloadRom,
    Tick,
    Play,
    Pause,
    PlayPause,
}

impl Display for MetaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(state) => write!(f, ":reset {}", match state {
                None => String::new(),
                Some(state) => format!("{}", state),
            }),
            Self::LoadRom(path, address) => {
                if let Some(address) = address {
                    write!(f, ":load {} {}", path, ui::util::Address::from(address))
                } else {
                    write!(f, ":load {}", path)
                }
            }
            Self::DumpMachine(path) => write!(f, ":dump {}", path),
            Self::LoadMachine(path) => write!(f, ":load-machine {}", path),
            Self::UnloadRom => write!(f, ":unload"),
            Self::Tick => write!(f, ":tick"),
            Self::Play => write!(f, ":play"),
            Self::Pause => write!(f, ":pause"),
            Self::PlayPause => write!(f, ":play-pause"),
        }
    }
}