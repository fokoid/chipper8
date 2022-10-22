use std::path::PathBuf;

use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::{Error, Result};
use chipper8::command::{Command, MachineState, MetaCommand};
use chipper8::machine::Machine;
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
            MetaCommand::LoadRom(name_or_path, address) => {
                let mut rom = match Rom::from_file(name_or_path) {
                    Ok(rom) => rom,
                    Err(_) => {
                        let mut path = PathBuf::new();
                        path.push("roms");
                        path.push(name_or_path);
                        path.set_extension("rom");
                        Rom::from_file(path)?
                    }
                };
                let address = address.unwrap_or(0x200).into();
                self.state.running = false;
                if let Some(mut rom) = self.state.unload_rom() {
                    self.machine.unload_rom(&mut rom);
                }
                self.machine.load_rom(&mut rom, address);
                self.state.load_rom(rom);
            }
            MetaCommand::UnloadRom => {
                self.state.running = false;
                if let Some(mut rom) = self.state.unload_rom() {
                    self.machine.unload_rom(&mut rom);
                }
            }
            MetaCommand::Tick => {
                self.state.running = false;
                self.tick()?;
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

    fn tick(&mut self) -> crate::Result<()> {
        let instruction = self.machine.next_instruction()?;
        self.state.command_history.append(&Command::Instruction(instruction), false);
        self.machine.tick()?;
        Ok(())
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.state.memory_tags.insert(MemoryTag::ProgramCounter,
                                      self.machine.program_counter..self.machine.program_counter + 2);
        self.state.memory_tags.insert(MemoryTag::Index,
                                      self.machine.index..self.machine.index + 1);
        self.ui.draw(ctx, &self.machine, &mut self.state);
        self.machine.key_buffer = self.state.key_capture.key();
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
                if let Err(error) = self.tick() {
                    self.state.error = Some(error);
                    self.state.running = false;
                    if let Error::InvalidOpCode(_) = self.state.error.as_ref().unwrap() {
                        if self.state.skip_unknown_opcode {
                            self.machine.program_counter += 2;
                            self.state.running = true;
                        }
                    }
                } else {
                    self.state.error = None;
                }
            }
            ctx.request_repaint_after(self.state.frame_time());
        }
    }
}