use egui_extras::{self, Size, TableBuilder};

use crate::ui::util::MonoLabel;

// todo: figure out how to make the outer Vec an Iterator
pub trait TabularData {
    fn header(&self) -> Option<Vec<MonoLabel>> { None }
    fn rows(&self) -> Vec<Vec<MonoLabel>>;

    fn total_rows(&self) -> usize { self.rows().len() }
    fn display_rows(&self) -> usize { self.total_rows() }
}

// todo: return a response
pub fn build(builder: TableBuilder, size: Vec<f32>, data: impl TabularData) {
    let mut data_iter = data.rows().into_iter();
    size.iter().fold(
        builder,
        |builder, size| builder.column(Size::exact(*size)),
    ).header(24.0, |row| {
        if let Some(header) = data.header() {
            header.into_iter().fold(row, |mut row, label| {
                row.col(|ui| { ui.add(label); });
                row
            });
        };
    }).body(|body| {
        body.rows(
            18.0,
            data.display_rows(),
            |_, mut row| {
                if let Some(record) = data_iter.next() {
                    record.into_iter().for_each(|cell| {
                        row.col(|ui| { ui.add(cell); });
                    })
                }
            },
        );
    });
}