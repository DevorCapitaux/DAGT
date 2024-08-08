use super::{binds::*, types::*};
use crate::error::{Error, Result};
use std::ffi::c_void;

#[derive(Debug, Clone, Copy)]
pub struct GlVAttrib {
    pub index: u32,
    pub size: i32,
    pub el_type: GlElType,
    pub normalize: bool,
    pub stride: i32,
    pub offset: i32,
}

pub trait GlVertex {
    fn get_attrs() -> &'static [GlVAttrib];
}

#[derive(Debug, Clone, Copy)]
pub struct GlVArray {
    array: GLuint,
}

impl GlVArray {
    pub fn create() -> GlVArray {
        unsafe {
            let mut array = 0;
            glGenVertexArrays(1, &mut array);

            GlVArray { array }
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.array);
        }
    }

    pub fn add_v_buffer<T: GlVertex>(&self, v: &[T]) -> Result<()> {
        unsafe {
            self.bind();

            let mut vbo = 0;
            glGenBuffers(1, &mut vbo);
            glBindBuffer(GlBuffTarget::ArrayBuffer.into(), vbo);
            let v_size = std::mem::size_of_val(&v[0]);
            glBufferData(
                GlBuffTarget::ArrayBuffer.into(),
                (v_size * v.len()) as GLsizeiptr,
                v.as_ptr() as *const c_void,
                GlBuffUsage::StaticDraw.into(),
            );
            Error::gl("glBufferData")?;

            for attr in T::get_attrs() {
                glVertexAttribPointer(
                    attr.index,
                    attr.size,
                    attr.el_type.into(),
                    attr.normalize as u8,
                    attr.stride,
                    attr.offset as *const c_void,
                );
                Error::gl("glVertexAttribPointer")?;
                glEnableVertexAttribArray(attr.index);
                Error::gl("glEnableVertexAttribArray")?;
            }

            Ok(())
        }
    }

    pub fn add_e_buffer(&self, i: &[u32]) -> Result<()> {
        unsafe {
            glBindVertexArray(self.array);

            let mut ebo = 0;
            glGenBuffers(1, &mut ebo);
            glBindBuffer(GlBuffTarget::ElementArrayBuffer.into(), ebo);
            glBufferData(
                GlBuffTarget::ElementArrayBuffer.into(),
                (std::mem::size_of_val(i)) as GLsizeiptr,
                i.as_ptr() as *const c_void,
                GlBuffUsage::StaticDraw.into(),
            );
            Error::gl("glBufferData")
        }
    }
}
