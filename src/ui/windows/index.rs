use egui::{Slider, Ui};
use egui_extras::TableBuilder;

use chipper8::machine::{DrawOptions, Machine};

use crate::State;
use crate::ui::util::{MemoryDisplay, MonoLabel, TabularData};
use crate::ui::util::table::{ColumnSpec, TableSpec};

use super::WindowContent;

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
    table_spec: TableSpec,
}

impl Index {
    pub fn new() -> Self {
        Self {
            display: MemoryDisplay::new(8, 16),
            buffer: [0; 128],
            draw_height: 15,
            table_spec: TableSpec {
                show_header: false,
                enable_context_menu: false,
                columns: vec![
                    ColumnSpec::fixed("Label", 40.0),
                    ColumnSpec::fixed("Hex", 40.0),
                    ColumnSpec::fixed("Decimal", 40.0),
                ],
            },
        }
    }
}

impl WindowContent for Index {
    fn name(&self) -> &'static str {
        "Index"
    }

    fn ui(&mut self, ui: &mut Ui, machine: &Machine, _state: &mut State) {
        self.buffer.fill(0);
        let height = self.draw_height % 16;
        DrawOptions::new(
            &machine.memory[machine.index..machine.index + height],
            &mut self.buffer,
            [8, 16],
        ).draw();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.table_spec.build(
                    TableBuilder::new(ui)
                        .resizable(false)
                        .scroll(false),
                    IndexHelper { machine },
                );
                ui.add(Slider::new(&mut self.draw_height, 0..=15));
            });
            self.display.ui(ui, &self.buffer);
        });
    }
}