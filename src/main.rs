use std::path::PathBuf;
use std::time::Duration;

use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::machine::Machine;
use chipper8::Result;
use chipper8::ui::{KeyCapture, Rom};
use chipper8::ui::windows::Display;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    rom: PathBuf,

    #[arg(short, long, default_value_t = 60)]
    fps: u64,
}

impl Args {
    fn frame_time(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.fps)
    }
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    args.rom.set_extension("rom");
    let mut native_options = NativeOptions::default();
    native_options.resizable = true;
    native_options.initial_window_size = Some(Vec2 { x: 280.0, y: 140.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(EmulatorApp::new(cc, args))));
    Ok(())
}

struct EmulatorApp {
    machine: Machine,
    last_time: f64,
    display: Display,
    args: Args,
    key_capture: KeyCapture,
}

impl EmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        let mut rom = Rom::from_file(&args.rom).unwrap();
        let mut machine = Machine::new();
        machine.load_rom(&mut rom, 0x200);
        println!("CHIPPER-8: running ROM '{}'.", rom.name);
        Self {
            machine,
            last_time: 0.0,
            display: Display::minimal(),
            args,
            key_capture: KeyCapture::new(),
        }
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx, |ui| {
                self.display.ui_stateless(ui, &mut self.machine);
                self.key_capture.update(ui);
            },
        );
        self.machine.key_buffer = self.key_capture.key();
        if ctx.input().time - self.last_time > self.args.frame_time().as_secs_f64() {
            self.last_time = ctx.input().time;
            let next_instruction = self.machine.next_instruction().unwrap();
            println!("Executing: {}", next_instruction);
            self.machine.step().unwrap();
        }
        ctx.request_repaint_after(self.args.frame_time());
    }
}