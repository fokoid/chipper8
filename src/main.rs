use eframe::NativeOptions;
use chipper8::instructions::{Command, MetaCommand};
use egui::{Color32, Frame, Stroke, TextStyle, Vec2};
use egui::widgets::TextEdit;
use egui::style::Margin;
use egui::widget_text::RichText;
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite, RingBufferRead};

// hard coded based on current (also hard coded) UI element sizes
const REPL_HISTORY_SIZE: usize = 16;

fn main() {
    let machine = chipper8::machine::Machine::new();
    println!("Hello, {:?}!", machine);

    let mut native_options = NativeOptions::default();
    native_options.resizable = false;
    native_options.initial_window_size = Some(Vec2 { x: 640.0, y: 375.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
}


struct ReplApp {
    user_input: String,
    history: AllocRingBuffer<Command>,
}

impl ReplApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            user_input: String::new(),
            history: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE),
        }
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("console")
            .resizable(false)
            .min_width(160.0)
            .max_width(160.0)
            .frame(Frame::default().stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("history")
                    .resizable(false)
                    .min_height(335.0)
                    .max_height(335.0)
                    .frame(Frame::none().inner_margin(Margin::symmetric(5.0, 5.0)))
                    .show_inside(ui, |ui| {
                        use egui_extras::{Size, TableBuilder};

                        let table = TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(40.0))
                            .column(Size::exact(100.0))
                            .resizable(false)
                            .scroll(false)
                            .stick_to_bottom(true);
                        table.body(|mut body| {
                            for command in self.history.iter() {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(RichText::new(match command.opcode() {
                                            None => String::from("META"),
                                            Some(opcode) => format!("{}", opcode),
                                        }).monospace().size(16.0));
                                    });
                                    row.col(|ui| { ui.label(RichText::new(format!("{}", command)).monospace().size(16.0)); });
                                });
                            };
                        });
                    });
                egui::TopBottomPanel::bottom("input")
                    .resizable(false)
                    .min_height(30.0)
                    .max_height(30.0)
                    .frame(Frame::default()
                        .inner_margin(Margin::symmetric(5.0, 0.0))
                        .fill(Color32::DARK_GRAY))
                    .show_inside(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            let input = ui.add(TextEdit::singleline(&mut self.user_input)
                                .font(TextStyle::Monospace)
                                .desired_width(250.0));
                            if input.lost_focus() {
                                match Command::parse(self.user_input.as_str().into()) {
                                    Ok(None) => {},
                                    Ok(Some(command)) => {
                                        self.history.push(command);
                                    },
                                    Err(error) => {
                                        println!("{:?}", error);
                                    },
                                };
                                self.user_input.clear();
                            };
                            input.request_focus();
                        });
                    });
            });
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
            ui.label("machine goes here");
        });
    }
}