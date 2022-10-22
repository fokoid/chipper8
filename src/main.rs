use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::thread;

use clap::Parser;
use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::{Error, Result};
use chipper8::machine::Machine;
use chipper8::ui::{KeyCapture, Rom};
use chipper8::ui::windows::Display;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // todo: split up GUI and emulator args
    #[arg(index = 1)]
    rom: PathBuf,

    #[arg(short, long, default_value_t = 60)]
    fps: u64,

    #[arg(long, default_value_t = false)]
    headless: bool,

    #[arg(long)]
    dump: Option<PathBuf>,
}

impl Args {
    fn frame_time(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.fps)
    }
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    args.rom.set_extension("rom");
    let mut emulator = Emulator::new(args)?;
    if !emulator.args.headless {
        let mut native_options = NativeOptions::default();
        native_options.resizable = true;
        native_options.run_and_return = false;
        native_options.initial_window_size = Some(Vec2 { x: 280.0, y: 140.0 });
        eframe::run_native("CHIPPER-8", native_options,
                           Box::new(|cc| Box::new(EmulatorApp::new(cc, emulator))));
    } else {
        // no display: useful for testing when combined with state dump
        while !emulator.terminated {
            emulator.tick();
            thread::sleep(emulator.args.frame_time());
        }
        if let Some(dump) = &emulator.args.dump {
            eprintln!("Writing final machine state to '{}'", dump.display());
            todo!("Implement serde for machine");
        }
    }
    Ok(())
}

struct Emulator {
    machine: Machine,
    last_time: Instant,
    terminated: bool,
    args: Args,
}

impl Emulator {
    fn new(args: Args) -> Result<Self> {
        let mut rom = Rom::from_file(&args.rom).unwrap();
        let mut machine = Machine::new();
        machine.load_rom(&mut rom, 0x200);
        println!("CHIPPER-8: running ROM '{}'.", rom.name);
        Ok(Self {
            machine,
            last_time: Instant::now(),
            terminated: false,
            args,
        })
    }

    fn tick(&mut self) {
        if self.terminated {
            return;
        }
        let current_time = Instant::now();
        if current_time - self.last_time > self.args.frame_time() {
            self.last_time = current_time;
            match self.machine.next_instruction() {
                Ok(instruction) => {
                    println!("Executing: {}", instruction);
                    match self.machine.tick() {
                        Err(Error::MachineExit) => { self.terminated = true; }
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("Error: {:?}", error);
                            panic!("");
                        }
                    }
                }
                Err(error @ Error::InvalidOpCode(_)) => {
                    eprintln!("Error: {:?}", error);
                    self.terminated = true;
                }
                _ => {}
            }
        }
    }
}

struct EmulatorApp {
    emulator: Emulator,
    display: Display,
    key_capture: KeyCapture,
}

impl EmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>, emulator: Emulator) -> Self {
        Self {
            emulator,
            display: Display::minimal(),
            key_capture: KeyCapture::new(),
        }
    }

}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx, |ui| {
                self.display.ui_stateless(ui, &mut self.emulator.machine);
                self.key_capture.update(ui);
            },
        );
        self.emulator.machine.key_buffer = self.key_capture.key();
        self.emulator.tick();
        ctx.request_repaint_after(self.emulator.args.frame_time());
    }
}