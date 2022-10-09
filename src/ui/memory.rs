use egui::{Color32, ColorImage, Context, Frame, TextureFilter, TextureHandle};
use egui::style::Margin;
use chipper8::machine::{self, Machine};

pub struct Memory {
    display: Option<TextureHandle>,
    mem_display: Option<TextureHandle>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            display: None,
            mem_display: None,
        }
    }

    /// build color image from machine video memory
    fn video_image(machine: &Machine) -> ColorImage {
        let mut display = ColorImage::new([machine::DISPLAY_WIDTH * 4, machine::DISPLAY_HEIGHT * 4], Color32::BLACK);
        for x in 0..machine::DISPLAY_WIDTH {
            for y in 0..machine::DISPLAY_HEIGHT {
                for i in 0..4 {
                    for j in 0..4 {
                        let [u, v] = [4 * x + i, 4 * y + j];
                        display.pixels[u + 4 * v * machine::DISPLAY_WIDTH] = Color32::from_gray(machine.display[x + y * machine::DISPLAY_WIDTH]);
                    }
                }
            }
        }
        display
    }

    /// build color image from machine video memory
    fn mem_image(machine: &Machine) -> ColorImage {
        let mut mem_display = ColorImage::new([ 64 * 4, 64 * 4], Color32::BLACK);
        for x in 0..64 {
            for y in 0..64 {
                for i in 0..4 {
                    for j in 0..4 {
                        let [u, v] = [4 * x + i, 4 * y + j];
                        mem_display.pixels[u + 4 * v * 64] = Color32::from_gray(machine.memory[x + y * 64]);
                    }
                }
            }
        }
        mem_display
    }

    pub fn draw(&mut self, ctx: &Context, machine: &Machine) {
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
                texture.set(Self::video_image(machine), TextureFilter::Linear);
                let size = texture.size_vec2();
                ui.image(texture, size);

                let mem_texture = self.mem_display.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        "mem-display",
                        ColorImage::new([64 * 4, 64 * 4], Color32::BLACK),
                        TextureFilter::Linear,
                    )
                });
                mem_texture.set(Self::mem_image(machine), TextureFilter::Linear);
                let size = mem_texture.size_vec2();
                ui.image(mem_texture, size);
            });
    }
}