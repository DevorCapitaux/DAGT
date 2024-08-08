#[derive(Clone, Copy, Debug)]
pub struct Color {
    color: [f32; 4],
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            color: [
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
                1.0,
            ],
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            color: [
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
                alpha as f32 / 255.0,
            ],
        }
    }

    pub(super) fn as_arr(&self) -> &[f32; 4] {
        &self.color
    }

    pub fn white() -> Color {
        Color::rgb(255, 255, 255)
    }

    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgb(0, 0, 0)
    }
}
