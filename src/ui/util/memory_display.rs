use egui::{Response, RichText, TextureHandle, TextureOptions, Ui};

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

    pub fn ui(&mut self, ui: &mut Ui, memory: &[u8], force_on: Vec<usize>, build_label_items: impl Fn(usize) -> Vec<RichText>) -> Response {
        let texture = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture(
                "display",
                self.image_builder.build_empty(),
                TextureOptions::LINEAR,
            )
        });
        texture.set(
            self.image_builder.build_from_memory(memory, force_on),
            TextureOptions::LINEAR,
        );
        let size = texture.size_vec2();
        let response = ui.image(texture, size);
        if let Some(cursor) = response.hover_pos() {
            let top = response.rect.min;
            let grid_pos = (cursor - top) / (self.image_builder.pixel_size as f32);
            let x = grid_pos.x as usize;
            let y = grid_pos.y as usize;
            let index = y * self.image_builder.width + x;
            if index >= self.image_builder.size() { return response; }
            let label_items = build_label_items(index);
            if !label_items.is_empty() {
                response.on_hover_ui_at_pointer(|ui| {
                    label_items.into_iter().for_each(|label_item| { ui.label(label_item); });
                })
            } else {
                response
            }
        } else {
            response
        }
    }
}
