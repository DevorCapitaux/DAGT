use super::{binds::*, types::*};
use crate::error::{Error, Result};
use std::ffi::CString;

pub struct GlShader {
    pub(super) shader: GLuint,
}

impl GlShader {
    pub fn create(shader_type: GlShaderType, source: &str) -> Result<GlShader> {
        unsafe {
            let shader = glCreateShader(shader_type.into());
            Error::gl("glCreateShader")?;

            let src = CString::new(source).unwrap();
            glShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
            Error::gl("glShaderSource")?;

            Ok(GlShader { shader })
        }
    }

    pub fn compile(&self) -> Result<()> {
        unsafe {
            glCompileShader(self.shader);

            let mut status = 0;
            glGetShaderiv(
                self.shader,
                GlShaderParam::CompileStatus.into(),
                &mut status,
            );
            if status == GL_FALSE {
                let mut length = 0;
                glGetShaderiv(
                    self.shader,
                    GlShaderParam::InfoLogLength.into(),
                    &mut length,
                );
                let mut msg: Vec<u8> = Vec::with_capacity(length as usize);
                glGetShaderInfoLog(
                    self.shader,
                    length,
                    &mut length,
                    msg.as_mut_ptr() as *mut i8,
                );
                msg.set_len(length as usize);
                // TODO: write shader's name here
                Err(Error::compile_shader("", String::from_utf8(msg).unwrap()))
            } else {
                Ok(())
            }
        }
    }
}
