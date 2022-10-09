use egui::{Color32, ColorImage};

pub struct ImageBuilder {
    width: usize,
    height: usize,
    pixel_size: usize,
    background: Color32,
}

impl ImageBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixel_size: 4,
            background: Color32::BLACK,
        }
    }

    pub fn build_empty(&self) -> ColorImage {
        ColorImage::new(self.pixel_size(), self.background)
    }

    pub fn build_from_memory(&self, memory: &[u8]) -> ColorImage {
        let mut image = self.build_empty();
        // todo: check memory bounds
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(
                    &mut image,
                    &[x, y],
                    Color32::from_gray(memory[x + y * self.width])
                );
            }
        };
        image
    }

    fn size(&self) -> [usize; 2] { [self.width, self.height] }
    fn pixel_width(&self) -> usize { self.width * self.pixel_size }
    fn pixel_height(&self) -> usize { self.height * self.pixel_size }
    fn pixel_size(&self) -> [usize; 2] { [self.pixel_width(), self.pixel_height()] }

    fn pixel_transform(&self, x: usize, pixel_offset: usize) -> usize { self.pixel_size * x + pixel_offset }
    fn pixel_pos(&self, pos: &[usize; 2], pixel_offset: &[usize; 2]) -> [usize; 2] {
        [
            self.pixel_transform(pos[0], pixel_offset[0]),
            self.pixel_transform(pos[1], pixel_offset[1]),
        ]
    }

    fn memory_offset(&self, pos: &[usize; 2], pixel_offset: &[usize; 2]) -> usize {
        let [u, v] = self.pixel_pos(pos, pixel_offset);
        u + self.pixel_width() * v
    }

    fn set_pixel(&self, image: &mut ColorImage, pos: &[usize; 2], color: Color32) {
        for j in 0..self.pixel_size {
            for i in 0..self.pixel_size {
                image.pixels[self.memory_offset(pos, &[i, j])] = color;
            }
        }
    }
}

