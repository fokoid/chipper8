use egui::Ui;
use egui_extras::{self, Size, TableBuilder, TableRow};

use crate::ui::util::MonoLabel;

// todo: figure out how to make the outer Vec an Iterator
pub trait TabularData {
    fn rows(&self) -> Vec<Vec<MonoLabel>>;

    fn total_rows(&self) -> usize { self.rows().len() }
    fn display_rows(&self) -> usize { self.total_rows() }
}

#[derive(Clone)]
pub struct ColumnSpec {
    pub name: String,
    pub visible: bool,
    pub size: Size,
}

impl ColumnSpec {
    pub fn fixed(name: impl Into<String>, size: f32) -> Self {
        Self {
            name: name.into(),
            visible: true,
            size: Size::exact(size),
        }
    }
}

#[derive(Clone)]
pub struct TableSpec {
    pub columns: Vec<ColumnSpec>,
    pub show_header: bool,
    pub enable_context_menu: bool,
}

impl TableSpec {
    pub fn new(columns: Vec<ColumnSpec>) -> Self {
        Self {
            columns,
            show_header: true,
            enable_context_menu: true,
        }
    }

    // todo: return a response
    pub fn build(&mut self, builder: TableBuilder, data: impl TabularData) {
        let mut data_iter = data.rows().into_iter();
        // If table spec is changed midway through, the TableBuilder can panic, since the number of
        // allocated columns may not match the number of header/row cells. We therefore make a copy
        // of the spec used for drawing and mutate the original.
        let draw_spec = self.clone();

        draw_spec.columns.iter().fold(
            builder,
            |builder, col_spec| {
                if col_spec.visible {
                    builder.column(col_spec.size)
                } else {
                    builder
                }
            },
        ).header(24.0, |row| {
            if !draw_spec.show_header { return; }
            draw_spec.columns.iter().fold(row, |mut row, col_spec| {
                column_header_ui(&mut row, self, col_spec);
                row
            });
        }).body(|body| {
            body.rows(
                18.0,
                data.display_rows(),
                |_, mut row| {
                    if let Some(record) = data_iter.next() {
                        draw_spec.columns.iter().zip(record.into_iter()).for_each(|(col_spec, content)| {
                            column_cell_ui(&mut row, self, col_spec, content);
                        })
                    }
                },
            );
        });
    }
}

fn column_toggle_menu_ui(ui: &mut Ui, table_spec: &mut TableSpec) {
    for column in table_spec.columns.iter_mut() {
        if ui.checkbox(&mut column.visible, &column.name).clicked() {
            ui.close_menu();
        };
    };
    ui.separator();
    if ui.checkbox(&mut table_spec.show_header, "Header").clicked() {
        ui.close_menu();
    }
}

fn column_header_ui(row: &mut TableRow, table_spec: &mut TableSpec, col_spec: &ColumnSpec) {
    column_cell_ui(row, table_spec, col_spec, MonoLabel::new(&col_spec.name))
}

// todo: should this return a Response? but then need to move the if out of the function
fn column_cell_ui(row: &mut TableRow, table_spec: &mut TableSpec, col_spec: &ColumnSpec, content: MonoLabel) {
    if !&col_spec.visible { return; }
    let response = row.col(|ui| { ui.add(content); });
    if table_spec.enable_context_menu {
        response.context_menu(|ui| {
            column_toggle_menu_ui(ui, table_spec);
        })
    } else {
        response
    };
}