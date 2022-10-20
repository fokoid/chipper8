use egui::{Color32, ColorImage};

pub struct ImageBuilder {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub color_map: Vec<Color32>,
}

impl ImageBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        let mut color_map = Vec::new();
        (0..(width * height)).for_each(|_| color_map.push(Color32::WHITE));
        Self {
            width,
            height,
            pixel_size: 4,
            color_map,
        }
    }

    pub fn build_empty(&self) -> ColorImage {
        ColorImage::new(self.pixel_size(), Color32::BLACK)
    }

    pub fn build_from_memory(&self, memory: &[u8], force_on: Vec<usize>) -> ColorImage {
        let mut image = self.build_empty();
        // todo: check memory bounds
        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + y * self.width;
                let scale = if force_on.contains(&index) {
                    1.0
                } else {
                    memory[index] as f32 / 255.0
                };
                self.set_pixel(
                    &mut image,
                    &[x, y],
                    self.color_map[index].linear_multiply(scale),
                );
            }
        };
        image
    }

    pub fn size(&self) -> usize { self.width * self.height }
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

