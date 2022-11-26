use crate::{Error, Result};
use crate::command::{Command, MachineState, MetaCommand};
use crate::machine::Address;

use super::{Token, Tokens};

// todo: implement for Command not Option<Command>
impl TryFrom<Tokens<'_>> for Option<Command> {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        match &tokens.peek() {
            None | Some(Token::None) => Ok(None),
            Some(Token::Meta(_)) => Ok(Some(Command::Meta(tokens.try_into()?))),
            _ => Ok(Some(Command::Instruction(tokens.try_into()?))),
        }
    }
}

impl TryFrom<Tokens<'_>> for MetaCommand {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Meta(":reset")) => Ok(MetaCommand::Reset(tokens.try_into()?)),
            Some(Token::Meta(":load")) => match tokens.next() {
                Some(Token::Other(name_or_path)) => {
                    let name_or_path = String::from(name_or_path);
                    match tokens.next() {
                        Some(token) => {
                            let address = Address::try_from(token)?;
                            Ok(MetaCommand::LoadRom(name_or_path, Some(address)))
                        }
                        None => Ok(MetaCommand::LoadRom(name_or_path, None)),
                    }
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(":load requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(":load requires a path"))),
            },
            Some(Token::Meta(":unload")) => Ok(MetaCommand::UnloadRom),
            Some(Token::Meta(":dump")) => match tokens.next() {
                Some(Token::Other(name_or_path)) => {
                    Ok(MetaCommand::DumpMachine(name_or_path.into()))
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(":dump requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(":dump requires a path"))),
            },
            Some(Token::Meta(":load-machine")) => match tokens.next() {
                Some(Token::Other(name_or_path)) => {
                    Ok(MetaCommand::LoadMachine(name_or_path.into()))
                }
                Some(x) => Err(Error::MetaSyntaxError(format!(":load-machine requires a path but got {:?}", x))),
                None => Err(Error::MetaSyntaxError(format!(":load-machine requires a path"))),
            },
            Some(Token::Meta(":tick")) => Ok(MetaCommand::Tick),
            Some(Token::Meta(":play")) => Ok(MetaCommand::Play),
            Some(Token::Meta(":pause")) => Ok(MetaCommand::Pause),
            Some(Token::Meta(":play-pause")) => Ok(MetaCommand::PlayPause),
            Some(Token::Meta(s)) => Err(Error::MetaSyntaxError(format!("invalid meta command '{}'", s))),
            s => Err(Error::MetaSyntaxError(format!("expected meta command token but found '{:?}'", s))),
        }
    }
}

impl TryInto<Option<MachineState>> for Tokens<'_> {
    type Error = Error;

    fn try_into(mut self) -> Result<Option<MachineState>> {
        match self.next() {
            Some(Token::Other("demo")) => Ok(Some(MachineState::Demo)),
            None => Ok(None),
            Some(x) => Err(Error::MetaSyntaxError(format!("not a valid machine state identifier: {:?}", x))),
        }
    }
}

