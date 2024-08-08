#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Resize {
    #[default]
    Fixed,
    Hug,
    Fill,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Constraints {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub horizontal_resize: Resize,
    pub verticle_resize: Resize,
}

impl Constraints {
    pub fn check(mut self) -> Self {
        self.width = self.width.clamp(0, i32::MAX);
        self.height = self.height.clamp(0, i32::MAX);
        self
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if width > 0 && height > 0 {
            self.width = width;
            self.height = height;
        }
    }

    pub fn in_box(&self, x: f64, y: f64) -> bool {
        x > self.x as f64
            && x < (self.x + self.width) as f64
            && y > self.y as f64
            && y < (self.y + self.height) as f64
    }
}

pub trait Draw {
    fn draw(&mut self, constraints: Constraints) -> bool;
}
