use egui::{TextureFilter, TextureHandle, Ui};

use crate::ui::util::image_builder::ImageBuilder;

pub struct MemoryDisplay {
    pub image_builder: ImageBuilder,
    texture: Option<TextureHandle>,
}

impl MemoryDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image_builder: ImageBuilder::new(width, height),
            texture: None,
        }
    }

    // todo: should return a response
    pub fn ui(&mut self, ui: &mut Ui, memory: &[u8], force_on: Vec<usize>) {
        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "display",
                self.image_builder.build_empty(),
                TextureFilter::Linear,
            )
        });
        texture.set(
            self.image_builder.build_from_memory(memory, force_on),
            TextureFilter::Linear,
        );
        let size = texture.size_vec2();
        ui.image(texture, size);
    }
}
