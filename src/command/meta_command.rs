use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

use crate::{Error, Result};

use super::machine_state::MachineState;
use super::tokens::{Token, Tokens};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetaCommand {
    Reset(Option<MachineState>),
    LoadRom(PathBuf, u16),
    UnloadRom,
    Step,
    Play,
    Pause,
    PlayPause,
}

impl MetaCommand {
    pub fn parse(mut tokens: Tokens) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Meta(".reset")) => Ok(MetaCommand::Reset(MachineState::parse(tokens)?)),
            Some(Token::Meta(".load")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    let mut path = PathBuf::new();
                    path.push(String::from(s));
                    path.set_extension("rom");
                    // default to address 200 which is what ROMs typically expect anyway
                    match tokens.next().unwrap_or(Token::Other("200")) {
                        Token::Other(s) => Ok(
                            MetaCommand::LoadRom(path, u16::from_str_radix(s, 16)?)
                        ),
                        x => Err(Error::MetaSyntaxError(format!(".load requires an address but got {:?}", x))),
                    }
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(".load requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(".load requires a path"))),
            },
            Some(Token::Meta(".unload")) => Ok(MetaCommand::UnloadRom),
            Some(Token::Meta(".step")) => Ok(MetaCommand::Step),
            Some(Token::Meta(".play")) => Ok(MetaCommand::Play),
            Some(Token::Meta(".pause")) => Ok(MetaCommand::Pause),
            Some(Token::Meta(".play-pause")) => Ok(MetaCommand::PlayPause),
            Some(Token::Meta(s)) => Err(Error::MetaSyntaxError(format!("invalid meta command '{}'", s))),
            s => Err(Error::MetaSyntaxError(format!("expected meta command token but found '{:?}'", s))),
        }
    }
}

impl Display for MetaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(state) => write!(f, ".reset {}", match state {
                None => String::new(),
                Some(state) => format!("{}", state),
            }),
            Self::LoadRom(path, address) => write!(f, ".load {} {:03X}", path.display(), address),
            Self::UnloadRom => write!(f, ".unload"),
            Self::Step => write!(f, ".step"),
            Self::Play => write!(f, ".play"),
            Self::Pause => write!(f, ".pause"),
            Self::PlayPause => write!(f, ".play-pause"),
        }
    }
}

