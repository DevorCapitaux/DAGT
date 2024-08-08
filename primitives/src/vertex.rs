use dagt_gl::gles::{
    types::GlElType,
    vertex::{GlVAttrib, GlVertex},
};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
}

impl Vertex {
    const ATTRIBS: &'static [GlVAttrib] = &[GlVAttrib {
        index: 0,
        size: 2,
        el_type: GlElType::Float,
        normalize: false,
        stride: 0,
        offset: 0,
    }];

    pub fn new(x: f32, y: f32) -> Self {
        Vertex { pos: [x, y] }
    }
}

impl GlVertex for Vertex {
    fn get_attrs() -> &'static [GlVAttrib] {
        Vertex::ATTRIBS
    }
}

impl Add for Vertex {
    type Output = Vertex;
    fn add(self, rhs: Self) -> Self::Output {
        Vertex::new(self.pos[0] + rhs.pos[0], self.pos[1] + rhs.pos[1])
    }
}

impl Sub for Vertex {
    type Output = Vertex;
    fn sub(self, rhs: Self) -> Self::Output {
        Vertex::new(self.pos[0] - rhs.pos[0], self.pos[1] - rhs.pos[1])
    }
}

impl Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f32) -> Self::Output {
        Vertex::new(self.pos[0] * rhs, self.pos[1] * rhs)
    }
}

impl From<&(f32, f32)> for Vertex {
    fn from(value: &(f32, f32)) -> Self {
        Vertex::new(value.0, value.1)
    }
}

impl From<(f32, f32)> for Vertex {
    fn from(value: (f32, f32)) -> Self {
        Vertex::new(value.0, value.1)
    }
}
