use std::path::PathBuf;

use clap::Parser;
use eframe::NativeOptions;
use egui::{Context, Vec2};

use chipper8::emulator::{Emulator, EmulatorConfig};
use chipper8::Result;
use chipper8::ui::KeyCapture;
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

impl From<&Args> for EmulatorConfig {
    fn from(args: &Args) -> Self {
        Self {
            rom_path: args.rom.clone(),
            fps: args.fps,
            dump_path: args.dump.clone(),
        }
    }
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    args.rom.set_extension("rom");
    let mut emulator = Emulator::new(EmulatorConfig::from(&args))?;
    if !args.headless {
        let mut native_options = NativeOptions::default();
        native_options.resizable = true;
        native_options.run_and_return = false;
        native_options.initial_window_size = Some(Vec2 { x: 280.0, y: 140.0 });
        eframe::run_native("CHIPPER-8", native_options,
                           Box::new(|cc| Box::new(EmulatorApp::new(cc, emulator))));
    } else {
        // no display: useful for testing when combined with state dump
        emulator.run()?;
    }
    Ok(())
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
        ctx.request_repaint_after(self.emulator.config.frame_time());
    }
}