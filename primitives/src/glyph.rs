use crate::path::Path;
use dagt_core::{Constraints, Draw};
use dagt_fonts::GlyphData;

pub struct Glyph {
    glyph: GlyphData,
}

impl Glyph {
    pub fn new(glyph: GlyphData) -> Glyph {
        Glyph { glyph }
    }
}

impl Draw for Glyph {
    fn draw(&mut self, constraints: Constraints) -> bool {
        let width = self.glyph.width;
        let height = self.glyph.height;
        let x_offset = -self.glyph.min_x;
        let y_offset = -self.glyph.min_y;

        let mut start = 0usize;
        for end in &self.glyph.contour_indices {
            let end = *end as usize;
            let mut v = Vec::new();

            for point in &self.glyph.points[start..=end] {
                let mut x = (point.x + x_offset + 1) as f32 / (width + 1) as f32;
                let mut y = (point.y + y_offset) as f32 / (height + 1) as f32;
                x = x * 2.0 - 1.0;
                y = y * 2.0 - 1.0;
                v.push((x, y).into());
            }
            start = end + 1;

            Path::new(&v, true).draw(Constraints { ..constraints });
        }

        true
    }
}
