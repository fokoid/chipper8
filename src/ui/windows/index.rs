use egui::{Slider, Ui};
use egui_extras::TableBuilder;

use chipper8::machine::{DrawOptions, Machine};

use crate::ui::util::{MemoryDisplay, MonoLabel, table, TabularData};

use super::Windowed;

struct IndexHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for IndexHelper<'a> {
    fn rows(&self) -> Vec<Vec<MonoLabel>> {
        vec![
            vec![
                MonoLabel::new("Addr"),
                MonoLabel::new(format!("{:03X}", self.machine.index)),
                MonoLabel::new(format!("{:04}", self.machine.index)),
            ],
            vec![
                MonoLabel::new("Byte"),
                MonoLabel::new(format!(" {:02X}", self.machine.at_index())),
                MonoLabel::new(format!(" {:03}", self.machine.at_index())),
            ],
        ]
    }
}

pub struct Index {
    display: MemoryDisplay,
    // 8x16 (all sprites are 8 pixels wide and up to 15 pixels tall)
    buffer: [u8; 128],
    draw_height: usize,
}

impl Index {
    pub fn new() -> Self {
        Self {
            display: MemoryDisplay::new(8, 16),
            buffer: [0; 128],
            draw_height: 15,
        }
    }
}

impl Windowed for Index {
    fn name(&self) -> &'static str {
        "Index"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine) {
        self.buffer.fill(0);
        let height = self.draw_height % 16;
        DrawOptions::new(
            &machine.memory[machine.index..machine.index + height],
            &mut self.buffer,
            [8, 16],
        ).draw();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                table::build(
                    TableBuilder::new(ui)
                        .resizable(false)
                        .scroll(false),
                    vec![40.0, 40.0, 40.0],
                    IndexHelper { machine },
                );
                ui.add(Slider::new(&mut self.draw_height, 0..=15));
            });
            self.display.ui(ui, &self.buffer);
        });
    }
}