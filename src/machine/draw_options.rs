use std::cmp::min;

pub struct DrawOptions<'a> {
    pos: [usize; 2],
    display_size: [usize; 2],
    source: &'a [u8],
    target: &'a mut [u8],
}

impl<'a> DrawOptions<'a> {
    pub fn new(source: &'a [u8], target: &'a mut [u8], display_size: [usize; 2]) -> Self {
        Self {
            pos: [0, 0],
            display_size,
            source,
            target,
        }
    }

    pub fn at(mut self, pos: [usize; 2]) -> Self {
        self.pos = pos;
        self
    }

    pub fn draw(self) {
        let bytes = self.source;
        let [x, y] = self.pos;
        let [display_width, display_height] = self.display_size;
        let height = self.source.len();
        for j in y..min(y + height, display_height) {
            let mut byte = bytes[j - y];
            for i in x..min(x + 8, display_width) {
                self.target[i + j * display_width] ^= if byte & 0b10000000 != 0 { 0xFF } else { 0 };
                byte = byte.rotate_left(1);
            }
        };
    }
}