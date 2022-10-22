use std::fmt::{Debug, Display, Formatter};

use crate::{Error, Result};
use crate::ui::util::Address;

use super::machine_state::MachineState;
use super::tokens::{Token, Tokens};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetaCommand {
    Reset(Option<MachineState>),
    LoadRom(String, Option<u16>),
    UnloadRom,
    Tick,
    Play,
    Pause,
    PlayPause,
}

impl MetaCommand {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Meta(":reset")) => Ok(MetaCommand::Reset(MachineState::parse(tokens)?)),
            Some(Token::Meta(":load")) => match tokens.next() {
                Some(Token::Other(name_or_path)) => {
                    let name_or_path = String::from(name_or_path);
                    // default to address 200 which is what ROMs typically expect anyway
                    match tokens.next() {
                        Some(Token::Other(s)) => {
                            let address = u16::from_str_radix(s, 16)?;
                            Ok(MetaCommand::LoadRom(name_or_path, Some(address)))
                        }
                        None => Ok(MetaCommand::LoadRom(name_or_path, None)),
                        Some(x) => Err(Error::MetaSyntaxError(format!(":load requires an address but got {:?}", x))),
                    }
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(":load requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(":load requires a path"))),
            },
            Some(Token::Meta(":unload")) => Ok(MetaCommand::UnloadRom),
            Some(Token::Meta(":tick")) => Ok(MetaCommand::Tick),
            Some(Token::Meta(":play")) => Ok(MetaCommand::Play),
            Some(Token::Meta(":pause")) => Ok(MetaCommand::Pause),
            Some(Token::Meta(":play-pause")) => Ok(MetaCommand::PlayPause),
            Some(Token::Meta(s)) => Err(Error::MetaSyntaxError(format!("invalid meta command '{}'", s))),
            s => Err(Error::MetaSyntaxError(format!("expected meta command token but found '{:?}'", s))),
        }
    }
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
                    write!(f, ":load {} {}", path, Address::from(*address))
                } else {
                    write!(f, ":load {}", path)
                }
            }
            Self::UnloadRom => write!(f, ":unload"),
            Self::Tick => write!(f, ":tick"),
            Self::Play => write!(f, ":play"),
            Self::Pause => write!(f, ":pause"),
            Self::PlayPause => write!(f, ":play-pause"),
        }
    }
}

