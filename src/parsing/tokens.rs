use std::str::SplitWhitespace;

use crate::{Error, Result};
use crate::machine::{Instruction, OpCode};
use crate::command::{Command, MetaCommand, MachineState};
use super::token::Token;

#[derive(Debug)]
pub struct Tokens<'a> {
    raw: SplitWhitespace<'a>,
    next: Option<Token<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.next.as_ref()
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.next.take();
        self.next = self.raw.next().map(|x| x.into());
        last
    }
}

impl<'a> From<Tokens<'a>> for String {
    fn from(tokens: Tokens) -> Self {
        let tokens: Vec<_> = tokens.map(String::from).collect();
        tokens.join(" ")
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(raw: &'a str) -> Self {
        let mut raw = raw.trim().split_whitespace();
        let next = raw.next().map(|x| x.into());
        Self { raw, next }
    }
}

impl TryFrom<Tokens<'_>> for OpCode {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        match tokens.next() {
            Some(Token::Hex(s)) => {
                Ok(Self(u16::from_str_radix(&s[2..], 16)?))
            }
            x => Err(Error::OpCodeSyntaxError(format!("{:?}", x))),
        }
    }
}

impl TryFrom<Tokens<'_>> for Instruction {
    type Error = Error;

    fn try_from(mut tokens: Tokens<'_>) -> Result<Self> {
        // todo: parse entire token stream
        match tokens.next() {
            Some(Token::Other("exit")) => Ok(Self::Exit),
            Some(Token::Other("cls")) => Ok(Self::ClearScreen),
            Some(Token::Other("jmp")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Jump(
                    // todo: bounds checking (12 bit address)
                    u16::from_str_radix(s, 16)?
                )),
                Some(x) => Err(Error::SyntaxError(format!("jmp requires an address but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("jmp requires an address"))),
            }
            Some(Token::Other("set")) => Ok(Instruction::Set { args: tokens.try_into()? }),
            Some(Token::Other("draw")) => Ok(Instruction::Draw { args: tokens.try_into()? }),
            Some(Token::Other("add")) => match tokens.next() {
                Some(Token::Other(s)) => {
                    // todo: bounds checking (12 bit address)
                    let register = u8::from_str_radix(s, 16)?;
                    match tokens.next() {
                        Some(Token::Other(s)) => Ok(Instruction::Add(
                            register,
                            u8::from_str_radix(s, 16)?,
                        )),
                        Some(x) => Err(Error::SyntaxError(format!("add requires a value but got {:?}", x))),
                        None => Err(Error::SyntaxError(format!("add requires a value"))),
                    }
                }
                Some(x) => Err(Error::SyntaxError(format!("add requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("add requires a register"))),
            },
            Some(Token::Other("index")) => match tokens.next() {
                Some(Token::Other("set")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::IndexSet(
                        // todo: bounds checking (12 bit address)
                        u16::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("index set requires an address but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("index set requires an address"))),
                },
                None | Some(_) => Err(Error::SyntaxError(String::from("allowed index sub commands: set"))),
            },
            Some(Token::Other("timer")) => match tokens.next() {
                Some(Token::Other("get")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::TimerGet(
                        u8::from_str_radix(s, 16)?
                    )),
                    Some(x) => Err(Error::SyntaxError(format!("timer get requires a register but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("timer get requires a register"))),
                },
                Some(_) => Err(Error::SyntaxError(String::from("allowed timer sub commands: get"))),
                None => Err(Error::SyntaxError(String::from("timer requires a sub command"))),
            },
            Some(Token::Other("font")) => match tokens.next() {
                Some(Token::Other(s)) => Ok(Instruction::Font(u8::from_str_radix(s, 16)?)),
                Some(x) => Err(Error::SyntaxError(format!("font requires a register but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("font requires a register"))),
            },
            Some(Token::Other("key")) => match tokens.next() {
                Some(Token::Other("await")) => match tokens.next() {
                    Some(Token::Other(s)) => Ok(Instruction::AwaitKey(u8::from_str_radix(s, 16)?)),
                    Some(x) => Err(Error::SyntaxError(format!("key await requires a register but got {:?}", x))),
                    None => Err(Error::SyntaxError(format!("key await requires a register"))),
                }
                Some(x) => Err(Error::SyntaxError(format!("key requires a subcommand but got {:?}", x))),
                None => Err(Error::SyntaxError(format!("key requires a subcommand"))),
            }
            x => Err(Error::SyntaxError(format!("{:?}", x))),
        }
    }
}

// todo: implement for Command not Option<Command>
impl TryFrom<Tokens<'_>> for Option<Command> {
    type Error = Error;

    fn try_from(mut tokens: Tokens) -> Result<Self> {
        match &tokens.peek() {
            None | Some(Token::None) => Ok(None),
            Some(Token::Hex(_)) => {
                let opcode = OpCode::try_from(tokens)?;
                Ok(Some(Command::Instruction(opcode.as_instruction()?)))
            }
            Some(token @ Token::Register(_)) => {
                Err(Error::SyntaxError(format!("unexpected token {:?}", token)))
            }
            Some(Token::Meta(_)) => Ok(Some(Command::Meta(tokens.try_into()?))),
            Some(Token::Other(_)) => Ok(Some(Command::Instruction(tokens.try_into()?))),
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

