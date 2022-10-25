use egui::WidgetText;

use crate::machine::Machine;
use crate::ui::util::TabularData;

use super::shared;

pub struct StackHelper<'a> {
    pub machine: &'a Machine,
}

impl<'a> TabularData for StackHelper<'a> {
    fn rows(&self) -> Vec<Vec<WidgetText>> {
        let mut rows: Vec<_> = self.machine.stack.data.iter().enumerate().map(|(index, address)| {
            let prefix = if index == self.machine.stack.pointer {
                format!(">{:01X}<", index)
            } else {
                format!(" {:01X} ", index)
            };
            if let Some(address) = address {
                shared::address_row(&prefix, address, self.machine)
            } else {
                vec![prefix.into(), "".into(), "".into(), "".into()]
            }
        }).collect();
        rows.push(vec![
            (if 16 == self.machine.stack.pointer { "> <" } else { "   " }).into(),
            "".into(),
            "".into(),
            "".into(),
        ]);
        rows
    }
}