use std::cmp::min;
use std::ops::BitXorAssign;

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

    pub fn draw(self) -> bool {
        let bytes = self.source;
        let [x, y] = self.pos;
        let [display_width, display_height] = self.display_size;
        let height = self.source.len();
        // track if any pixels get unset
        let mut pixel_off_flag = false;
        for j in y..min(y + height, display_height) {
            let mut byte = bytes[j - y];
            for i in x..min(x + 8, display_width) {
                let target = &mut self.target[i + j * display_width];
                let last = target.clone();
                target.bitxor_assign(if byte & 0b10000000 != 0 { 0xFF } else { 0 });
                if last != 0 && *target == 0 {
                    pixel_off_flag = true;
                }
                byte = byte.rotate_left(1);
            }
        };
        pixel_off_flag
    }
}