use egui::{Color32, Context, Frame, TextureFilter, TextureHandle, Ui};
use egui::style::Margin;

use chipper8::machine::{self, Machine};

use crate::ui::image_builder::ImageBuilder;

pub struct Memory {
    video: MemoryDisplay,
    system: MemoryDisplay,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            video: MemoryDisplay::new(machine::DISPLAY_WIDTH, machine::DISPLAY_HEIGHT),
            system: MemoryDisplay::new(64, 64),
        }
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine) {
        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin::same(5.0)).fill(Color32::DARK_GRAY))
            .show(ctx, |ui| {
                self.video.draw(ui, &machine.display);
                self.system.draw(ui, &machine.memory);
            });
    }
}

struct MemoryDisplay {
    image_builder: ImageBuilder,
    texture: Option<TextureHandle>,
}

impl MemoryDisplay {
    fn new(width: usize, height: usize) -> Self {
        Self {
            image_builder: ImageBuilder::new(width, height),
            texture: None,
        }
    }

    fn draw(&mut self, ui: &mut Ui, memory: &[u8]) {
        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "display",
                self.image_builder.build_empty(),
                TextureFilter::Linear,
            )
        });
        texture.set(
            self.image_builder.build_from_memory(memory),
            TextureFilter::Linear,
        );
        let size = texture.size_vec2();
        ui.image(texture, size);
    }
}
