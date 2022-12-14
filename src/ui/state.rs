use std::collections::BTreeMap;
use std::fs;
use std::ops::Range;
use std::path::Path;
use std::time::Duration;

use egui::Color32;

use crate::{Error, Result};
use crate::assembler::Tokens;
use crate::command::Command;
use crate::machine;

use super::command_history::CommandHistory;
use super::KeyCapture;

pub struct State {
    pub running: bool,
    pub skip_unknown_opcode: bool,
    pub command_history: CommandHistory,
    pub command_buffer: Option<Command>,
    pub error: Option<Error>,
    pub key_capture: KeyCapture,
    // when a text edit field has focus, do not send any key presses to the virtual keypad
    pub key_capture_suspended: bool,
    pub rom: Option<Rom>,
    pub memory_tags: BTreeMap<MemoryTag, Range<usize>>,
    pub frames_per_second: u64,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: false,
            skip_unknown_opcode: false,
            command_history: CommandHistory::new(),
            command_buffer: None,
            error: None,
            key_capture: KeyCapture::new(),
            key_capture_suspended: false,
            rom: None,
            memory_tags: BTreeMap::from([
                (MemoryTag::Reserved, 0..0x200),
                (MemoryTag::SystemFont, machine::config::FONT_RANGE),
            ]),
            // todo: is this really state or should it be machine 'config'?
            // (but for now the UI can't modify the machine directly so it lives here)
            frames_per_second: 60,
        }
    }

    pub fn frame_time(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.frames_per_second)
    }

    pub fn parse_command(&mut self, input: &str) {
        let tokens = Tokens::from(input);
        match tokens.try_into() {
            Ok(Some(command)) => {
                self.command_buffer.replace(command);
                self.error.take();
            }
            Ok(None) => {}
            Err(error) => {
                self.command_buffer.take();
                self.error.replace(error);
            }
        };
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.memory_tags.insert(MemoryTag::UserProgram { name: rom.name.clone() }, rom.loaded_range().unwrap());
        self.rom = Some(rom);
    }

    pub fn unload_rom(&mut self) -> Option<Rom> {
        let rom = self.rom.take()?;
        self.memory_tags.remove(&MemoryTag::UserProgram { name: rom.name.clone() });
        Some(rom)
    }

    pub fn error(&mut self) -> Option<&Error> {
        self.error.as_ref()
    }
}


#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum MemoryTag {
    // the order here determines priority: later variants are draw later over the top of prior ones
    Reserved,
    SystemFont,
    UserProgram { name: String },
    Index,
    ProgramCounter,
}

impl MemoryTag {
    pub fn color(&self) -> Color32 {
        match self {
            Self::Reserved => Color32::LIGHT_GRAY,
            Self::SystemFont => Color32::YELLOW,
            Self::UserProgram { name: _name } => Color32::RED,
            Self::ProgramCounter => Color32::WHITE,
            Self::Index => Color32::LIGHT_GREEN,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Reserved => String::from("System Reserved"),
            Self::SystemFont => String::from("System Fonts"),
            Self::UserProgram { name } => format!("User Program ({}.rom)", name),
            Self::ProgramCounter => String::from("Program Counter"),
            Self::Index => String::from("Index"),
        }
    }
}

// todo: clearly ROM doesn't belong in this module
pub struct Rom {
    pub name: String,
    pub bytes: Vec<u8>,
    pub loaded_at: Option<usize>,
}

impl Rom {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = fs::read(path.as_ref())?;
        Ok(Self {
            name: String::from(path.as_ref().file_name().unwrap().to_str().unwrap()),
            bytes,
            loaded_at: None,
        })
    }

    pub fn loaded_range(&self) -> Option<Range<usize>> {
        let start = self.loaded_at?;
        Some(start..start + self.bytes.len())
    }
}