use egui::Ui;
use egui_extras::{self, Size};

use crate::ui::util::MonoLabel;

// todo: figure out how to make the outer Vec an Iterator
pub trait TabularData {
    fn rows(&self) -> Vec<Vec<MonoLabel>>;
}

pub fn build(ui: &mut Ui, size: Vec<f32>, data: impl TabularData) {
    let num_cols = size.len();
    size.iter().fold(
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .scroll(false)
            .stick_to_bottom(true),
        |builder, size| builder.column(Size::exact(*size)),
    ).body(|mut body| {
        data.rows().into_iter().for_each(|record| {
            if record.len() != num_cols {
                panic!("table has {} columns but record has {}: {:?}",
                       num_cols,
                       record.len(),
                       record.iter().collect::<Vec<_>>());
            }
            body.row(18.0, |mut row| {
                record.into_iter().for_each(|cell| {
                    row.col(|ui| { ui.add(cell); });
                });
            })
        });
    });
}