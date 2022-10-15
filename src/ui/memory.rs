use egui::{Response, TextureFilter, TextureHandle, Ui};

use crate::ui::image_builder::ImageBuilder;

pub struct MemoryDisplay {
    image_builder: ImageBuilder,
    texture: Option<TextureHandle>,
}

impl MemoryDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image_builder: ImageBuilder::new(width, height),
            texture: None,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, memory: &[u8]) -> Response {
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
        ui.image(texture, size)
    }
}
