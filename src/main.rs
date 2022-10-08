use std::fs;
use std::time::Duration;
use eframe::NativeOptions;
use chipper8::machine::{self, Machine};
use chipper8::instructions::{Command, MetaCommand};
use egui::{Color32, ColorImage, Frame, Stroke, TextStyle, TextureFilter, TextureHandle, Vec2};
use egui::widgets::TextEdit;
use egui::style::Margin;
use egui::widget_text::RichText;
use egui_extras::{Size, TableBuilder};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

// hard coded based on current (also hard coded) UI element sizes
const REPL_HISTORY_SIZE: usize = 16;

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.resizable = false;
    native_options.initial_window_size = Some(Vec2 { x: 680.0, y: 395.0 });
    eframe::run_native("CHIPPER-8", native_options,
                       Box::new(|cc| Box::new(ReplApp::new(cc))));
}


struct ReplApp {
    user_input: String,
    history: AllocRingBuffer<Command>,
    machine: Machine,
    display: Option<TextureHandle>,
    mem_display: Option<TextureHandle>,
    running: bool,
    last_time: f64,
}

impl ReplApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            user_input: String::new(),
            history: AllocRingBuffer::with_capacity(REPL_HISTORY_SIZE),
            machine: Machine::demo(),
            display: None,
            mem_display: None,
            running: false,
            last_time: 0.0,
        }
    }
}

impl eframe::App for ReplApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("console")
            .resizable(false)
            .min_width(180.0)
            .max_width(180.0)
            .frame(Frame::default().stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("history")
                    .resizable(false)
                    .min_height(335.0)
                    .max_height(335.0)
                    .frame(Frame::none().inner_margin(Margin::symmetric(5.0, 5.0)))
                    .show_inside(ui, |ui| {
                        let table = TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(40.0))
                            .column(Size::exact(120.0))
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
                                        match &command {
                                            Command::Instruction(instruction) => {
                                                // user entered a machine instruction at the prompt
                                                // so we should suspend the VM main loop if running
                                                self.running = false;
                                                self.machine.execute(instruction)
                                            },
                                            Command::Meta(MetaCommand::Reset) => {
                                                self.running = false;
                                                self.machine.reset();
                                            },
                                            Command::Meta(MetaCommand::Load(path, address)) => {
                                                let bytes = fs::read(path).unwrap();
                                                self.machine.load(*address as usize, &bytes);
                                                self.machine.program_counter = *address as usize;
                                            }
                                            Command::Meta(MetaCommand::Step) => {
                                                self.machine.step().unwrap();
                                            },
                                            Command::Meta(MetaCommand::Play) => {
                                                self.running = true;
                                            },
                                            Command::Meta(MetaCommand::Pause) => {
                                                self.running = false;
                                            },
                                        };
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
        egui::SidePanel::right("vm-visualizer")
            .resizable(false)
            .min_width(230.0)
            .max_width(230.0)
            .frame(Frame::default()
                .inner_margin(Margin::symmetric(10.0, 5.0))
                .stroke(Stroke::new(2.0, Color32::DARK_GRAY)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.push_id(0, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(20.0))
                            .column(Size::exact(20.0))
                            .resizable(false)
                            .scroll(false)
                            .body(|mut body| {
                                for (index, value) in self.machine.registers.iter().enumerate() {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(format!("V{:1X}", index)).monospace().size(16.0));
                                        });
                                        row.col(|ui| { ui.label(RichText::new(format!("{:02X}", value)).monospace().size(16.0)); });
                                    });
                                };
                            });
                    });
                    ui.push_id(1, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Size::exact(40.0))
                            .resizable(false)
                            .scroll(false)
                            .body(|mut body| {
                                for index in 0..machine::STACK_SIZE {
                                    body.row(18.0, |mut row| {
                                        row.col(|ui| {
                                            let text = RichText::new(format!("{:04X}", self.machine.stack.data[index]))
                                                .monospace().size(16.0);
                                            ui.label(
                                                if index == self.machine.stack.pointer {
                                                    text.background_color(Color32::LIGHT_RED)
                                                } else { text }
                                            );
                                        });
                                    });
                                }
                            });
                    });
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("PC  {:04X} {:04X}", self.machine.program_counter, self.machine.at_program_counter())).monospace().size(16.0));
                        if let Ok(instruction) = self.machine.next_instruction() {
                            ui.label(RichText::new(format!("{}", instruction)).monospace().size(16.0));
                        };
                        ui.label(RichText::new(format!("IDX {:04X} {:04X}", self.machine.index, self.machine.at_index())).monospace().size(16.0));
                        ui.label(RichText::new(format!("DELAY {:02X}", self.machine.delay_timer)).monospace().size(16.0));
                        let sound_label = RichText::new(format!("SOUND {:02X}", self.machine.sound_timer)).monospace().size(16.0);
                        ui.label(if self.machine.sound_timer > 0 { sound_label.background_color(Color32::LIGHT_RED) } else { sound_label });
                    })
                });
            });
        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin::same(5.0)).fill(Color32::DARK_GRAY))
            .show(ctx, |ui| {
                let texture = self.display.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        "display",
                        ColorImage::new([machine::DISPLAY_WIDTH * 4, machine::DISPLAY_HEIGHT * 4], Color32::BLACK),
                        TextureFilter::Linear,
                    )
                });
                // build color image from machine video memory
                let mut display = ColorImage::new([machine::DISPLAY_WIDTH * 4, machine::DISPLAY_HEIGHT * 4], Color32::BLACK);
                for x in 0..machine::DISPLAY_WIDTH {
                    for y in 0..machine::DISPLAY_HEIGHT {
                        for i in 0..4 {
                            for j in 0..4 {
                                let [u, v] = [4 * x + i, 4 * y + j];
                                display.pixels[u + 4 * v * machine::DISPLAY_WIDTH] = Color32::from_gray(self.machine.display[x + y * machine::DISPLAY_WIDTH]);
                            }
                        }
                    }
                }
                texture.set(display, TextureFilter::Linear);
                let size = texture.size_vec2();
                ui.image(texture, size);

                let mem_texture = self.mem_display.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        "mem-display",
                        ColorImage::new([64 * 4, 64 * 4], Color32::BLACK),
                        TextureFilter::Linear,
                    )
                });
                // build color image from machine video memory
                let mut mem_display = ColorImage::new([ 64 * 4, 64 * 4], Color32::BLACK);
                for x in 0..64 {
                    for y in 0..64 {
                        for i in 0..4 {
                            for j in 0..4 {
                                let [u, v] = [4 * x + i, 4 * y + j];
                                mem_display.pixels[u + 4 * v * 64] = Color32::from_gray(self.machine.memory[x + y * 64]);
                            }
                        }
                    }
                }
                mem_texture.set(mem_display, TextureFilter::Linear);
                let size = mem_texture.size_vec2();
                ui.image(mem_texture, size);
            });
        // if VM main loop is running, and timer is up, execute next command
        if self.running {
            // todo make timing here configurable
            if ctx.input().time - self.last_time > 0.1 {
                self.last_time = ctx.input().time;
                self.machine.step().unwrap();
            }
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}