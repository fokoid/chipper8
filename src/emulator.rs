use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use crate::{Error, Result};
use crate::machine::Machine;
use crate::ui::Rom;

pub struct EmulatorConfig {
    pub rom_path: PathBuf,
    pub fps: u64,
    pub dump_path: Option<PathBuf>,
}

impl EmulatorConfig {
    pub fn frame_time(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.fps)
    }
}

pub struct Emulator {
    pub machine: Machine,
    pub last_time: Instant,
    pub terminated: bool,
    pub config: EmulatorConfig,
}

impl Emulator {
    pub fn new(config: EmulatorConfig) -> Result<Self> {
        let mut rom = Rom::from_file(&config.rom_path).unwrap();
        let mut machine = Machine::new();
        machine.load_rom(&mut rom, None);
        println!("CHIPPER-8: running ROM '{}'.", rom.name);
        Ok(Self {
            machine,
            last_time: Instant::now(),
            terminated: false,
            config,
        })
    }

    pub fn tick(&mut self) {
        if self.terminated {
            return;
        }
        let current_time = Instant::now();
        if current_time - self.last_time > self.config.frame_time() {
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

    pub fn run(&mut self) -> Result<()> {
        while !self.terminated {
            self.tick();
            thread::sleep(self.config.frame_time());
        }
        if let Some(dump) = &self.config.dump_path {
            eprintln!("Writing final machine state to '{}'", dump.display());
            fs::write(dump, serde_json::to_string(&self.machine)?)?;
        };
        Ok(())
    }
}

