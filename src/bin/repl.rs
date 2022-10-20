use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::command::{Command, MachineState, MetaCommand};
use chipper8::machine::Machine;
use chipper8::Result;
use chipper8::ui::{MemoryTag, Rom, State, Ui};

fn main() -> Result<()> {
    let mut native_options = NativeOptions::default();
    native_options.resizable = true;
    native_options.initial_window_size = Some(Vec2 { x: 1500.0, y: 700.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
    Ok(())
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
                if let Some(mut rom) = self.state.unload_rom() {
                    self.machine.unload_rom(&mut rom);
                }
                let mut rom = Rom::from_file(path)?;
                self.machine.load_rom(&mut rom, *address as usize);
                self.state.load_rom(rom);
            }
            MetaCommand::UnloadRom => {
                self.state.running = false;
                if let Some(mut rom) = self.state.unload_rom() {
                    self.machine.unload_rom(&mut rom);
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