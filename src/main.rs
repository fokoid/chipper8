mod ui;

use std::fs;
use eframe::NativeOptions;
use chipper8::machine::{self, Machine};
use chipper8::instructions::{Command, MetaCommand, MachineState};
use egui::{Context, Vec2};
use ui::Ui;

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.resizable = false;
    native_options.initial_window_size = Some(Vec2 { x: 765.0, y: 395.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
}

struct ReplApp {
    ui: Ui,
    machine: Machine,
    running: bool,
    last_time: f64,
}

impl ReplApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ui: Ui::new(),
            machine: Machine::new(),
            running: false,
            last_time: 0.0,
        }
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Instruction(instruction) => {
                // user entered a machine instruction at the prompt
                // so we should suspend the VM main loop if running
                self.running = false;
                self.machine.execute(instruction)
            }
            Command::Meta(meta) => self.execute_meta(meta),
        }
    }

    fn execute_meta(&mut self, command: &MetaCommand) {
        match command {
            MetaCommand::Reset(state) => {
                self.running = false;
                self.machine.reset();
                if let Some(state) = state {
                    match state {
                        MachineState::Demo => self.machine.demo(),
                    };
                };
            }
            MetaCommand::Load(path, address) => {
                let bytes = fs::read(path).unwrap();
                self.machine.load(*address as usize, &bytes);
                self.machine.program_counter = *address as usize;
            }
            MetaCommand::Step => {
                self.machine.step().unwrap();
            }
            MetaCommand::Play => {
                self.running = true;
            }
            MetaCommand::Pause => {
                self.running = false;
            }
        }
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.ui.draw(ctx, &self.machine) {
            Ok(None) => {}
            Ok(Some(command)) => self.execute(&command),
            Err(error) => {
                println!("{:?}", error);
            }
        };
        // if VM main loop is running, and timer is up, execute next command
        if self.running {
            // todo make timing here configurable
            if ctx.input().time - self.last_time > machine::FRAME_TIME.as_secs_f64() {
                self.last_time = ctx.input().time;
                let instruction = self.machine.next_instruction().unwrap();
                self.ui.add_history(&Command::Instruction(instruction), false);
                self.machine.step().unwrap();
            }
            ctx.request_repaint_after(machine::FRAME_TIME);
        }
    }
}