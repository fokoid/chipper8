use std::collections::BTreeMap;
use std::fs;
use std::ops::Range;
use std::time::Duration;

use eframe::NativeOptions;
use egui::{Color32, Context, Vec2};

use chipper8::instructions::{Command, Error, MachineState, MetaCommand, Result};
use chipper8::machine::{self, Machine};
use command_history::CommandHistory;
use ui::Ui;

mod ui;
mod command_history;

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.resizable = true;
    native_options.initial_window_size = Some(Vec2 { x: 1500.0, y: 700.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
}

pub struct Rom {
    name: String,
    bytes: Vec<u8>,
    loaded_at: Option<usize>,
}

impl Rom {
    fn from_file(filename: &str) -> Result<Self> {
        let name = if filename.ends_with(".rom") {
            &filename[..filename.len() - 4]
        } else { &filename };
        let bytes = fs::read(format!("{}.rom", name))?;
        Ok(Self {
            name: String::from(name),
            bytes,
            loaded_at: None,
        })
    }

    fn loaded_range(&self) -> Option<Range<usize>> {
        let start = self.loaded_at?;
        Some(start..start + self.bytes.len())
    }

    fn load(&mut self, address: usize, machine: &mut Machine, state: &mut State) {
        if self.loaded_at.is_some() {
            panic!("rom already loaded");
        }
        self.loaded_at = Some(address);
        machine.load(address, &self.bytes);
        machine.program_counter = address;
        state.memory_tags.insert(MemoryTag::UserProgram { name: self.name.clone() }, self.loaded_range().unwrap());
    }

    fn unload(&mut self, machine: &mut Machine, state: &mut State) {
        if self.loaded_at.is_none() {
            panic!("attempt to unload ROM that was never loaded");
        }
        machine.memory[self.loaded_range().unwrap()].fill(0);
        state.memory_tags.remove(&MemoryTag::UserProgram { name: self.name.clone() });
        self.loaded_at = None;
        // todo: move program counter?
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


pub struct State {
    pub running: bool,
    pub command_history: CommandHistory,
    pub command_buffer: Option<Command>,
    pub error: Option<Error>,
    pub keys: [bool; 16],
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
            command_history: CommandHistory::new(),
            command_buffer: None,
            error: None,
            keys: [false; 16],
            key_capture_suspended: false,
            rom: None,
            memory_tags: BTreeMap::from([
                (MemoryTag::Reserved, 0..0x200),
                (MemoryTag::SystemFont, machine::FONT_RANGE),
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
        match Command::parse(input.into()) {
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

    pub fn error(&mut self) -> Option<&Error> {
        self.error.as_ref()
    }
}

struct ReplApp {
    ui: Ui,
    machine: Machine,
    last_time: f64,
    state: State,
}

impl ReplApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ui: Ui::new(),
            machine: Machine::new(),
            last_time: 0.0,
            state: State::new(),
        }
    }

    fn execute(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Instruction(instruction) => {
                // user entered a machine instruction at the prompt
                // so we should suspend the VM main loop if running
                self.state.running = false;
                // todo: Machine::execute should also return result
                self.machine.execute(instruction);
                Ok(())
            }
            Command::Meta(meta) => self.execute_meta(meta),
        }
    }

    fn execute_meta(&mut self, command: &MetaCommand) -> Result<()> {
        match command {
            MetaCommand::Reset(state) => {
                self.state.running = false;
                self.machine.reset();
                if let Some(state) = state {
                    match state {
                        MachineState::Demo => self.machine.demo(),
                    };
                };
            }
            MetaCommand::Load(path, address) => {
                self.state.running = false;
                if let Some(mut rom) = self.state.rom.take() {
                    rom.unload(&mut self.machine, &mut self.state);
                }
                let mut rom = Rom::from_file(path)?;
                rom.load(*address as usize, &mut self.machine, &mut self.state);
                self.state.rom = Some(rom);
            }
            MetaCommand::UnloadRom => {
                self.state.running = false;
                if let Some(mut rom) = self.state.rom.take() {
                    rom.unload(&mut self.machine, &mut self.state);
                }
            }
            MetaCommand::Step => {
                self.state.running = false;
                self.step();
            }
            MetaCommand::Play => {
                self.state.running = true;
            }
            MetaCommand::Pause => {
                self.state.running = false;
            }
            MetaCommand::PlayPause => {
                self.state.running = !self.state.running;
            }
        };
        Ok(())
    }

    fn step(&mut self) {
        let instruction = self.machine.next_instruction().unwrap();
        self.state.command_history.append(&Command::Instruction(instruction), false);
        self.machine.step().unwrap();
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.state.memory_tags.insert(MemoryTag::ProgramCounter,
                                      self.machine.program_counter..self.machine.program_counter + 2);
        self.state.memory_tags.insert(MemoryTag::Index,
                                      self.machine.index..self.machine.index + 1);
        self.ui.draw(ctx, &self.machine, &mut self.state);
        if let Some(command) = &self.state.command_buffer.take() {
            self.state.command_history.append(command, true);
            match self.execute(command) {
                Ok(_) => {}
                Err(error) => {
                    self.state.error = Some(error);
                }
            };
        };
        // if VM main loop is running, and timer is up, execute next command
        if self.state.running {
            // todo make timing here configurable
            if ctx.input().time - self.last_time > self.state.frame_time().as_secs_f64() {
                self.last_time = ctx.input().time;
                self.step();
            }
            ctx.request_repaint_after(self.state.frame_time());
        }
    }
}