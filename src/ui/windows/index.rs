use egui::{Slider, Ui};

use chipper8::machine::{DrawOptions, Machine};

use crate::State;
use crate::ui::util::{MemoryDisplay, MonoLabel, TabularData, Address, Byte, Decimal};
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
                MonoLabel::new(Address::from(self.machine.index)),
                MonoLabel::new(Decimal::from(self.machine.index)),
            ],
            vec![
                MonoLabel::new("Byte"),
                MonoLabel::new(Byte::from(self.machine.at_index())),
                MonoLabel::new(Decimal::from(self.machine.at_index())),
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
            table_spec: TableSpec::new(
                vec![
                    ColumnSpec::fixed("Label", 40.0),
                    ColumnSpec::fixed("Hex", 40.0),
                    ColumnSpec::fixed("Decimal", 40.0),
                ],
            ).header(false).context_menu(false),
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
                self.table_spec.draw(ui, IndexHelper { machine });
                ui.add(Slider::new(&mut self.draw_height, 0..=15));
            });
            self.display.ui(ui, &self.buffer);
        });
    }
}