use std::fs;

use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::instructions::{Command, Error, MachineState, MetaCommand};
use chipper8::machine::{self, Machine};
use command_history::CommandHistory;
use ui::Ui;

mod ui;
mod command_history;

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.resizable = true;
    native_options.initial_window_size = Some(Vec2 { x: 1400.0, y: 800.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
}

pub struct State {
    pub running: bool,
    pub command_history: CommandHistory,
    pub command_buffer: Option<Command>,
    pub error: Option<Error>,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: false,
            command_history: CommandHistory::new(),
            command_buffer: None,
            error: None,
        }
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

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Instruction(instruction) => {
                // user entered a machine instruction at the prompt
                // so we should suspend the VM main loop if running
                self.state.running = false;
                self.machine.execute(instruction)
            }
            Command::Meta(meta) => self.execute_meta(meta),
        }
    }

    fn execute_meta(&mut self, command: &MetaCommand) {
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
                let bytes = fs::read(path).unwrap();
                self.machine.load(*address as usize, &bytes);
                self.machine.program_counter = *address as usize;
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
        }
    }

    fn step(&mut self) {
        let instruction = self.machine.next_instruction().unwrap();
        self.state.command_history.append(&Command::Instruction(instruction), false);
        self.machine.step().unwrap();
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.ui.draw(ctx, &self.machine, &mut self.state);
        if let Some(command) = &self.state.command_buffer.take() {
            self.state.command_history.append(command, true);
            self.execute(command);
        };
        // if VM main loop is running, and timer is up, execute next command
        if self.state.running {
            // todo make timing here configurable
            if ctx.input().time - self.last_time > machine::FRAME_TIME.as_secs_f64() {
                self.last_time = ctx.input().time;
                self.step();
            }
            ctx.request_repaint_after(machine::FRAME_TIME);
        }
    }
}