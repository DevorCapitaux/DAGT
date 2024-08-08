use super::{binds::*, shader::GlShader, types::*};
use crate::error::{Error, Result};
use std::ffi::CString;

pub struct GlProgram {
    program: GLuint,
}

impl GlProgram {
    pub fn create() -> Result<GlProgram> {
        unsafe {
            let program = glCreateProgram();
            if program == 0 {
                Err(Error::gl_err("glCreateProgram"))
            } else {
                Ok(GlProgram { program })
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.program);
        }
    }

    pub fn attach(&self, shader: &GlShader) -> Result<()> {
        unsafe {
            glAttachShader(self.program, shader.shader);
            Error::gl("glAttachShader")
        }
    }

    pub fn link(&self) -> Result<()> {
        unsafe {
            glLinkProgram(self.program);

            let mut status = 0;
            glGetProgramiv(self.program, GlProgramParam::LinkStatus.into(), &mut status);
            if status == GL_FALSE {
                let mut length = 0;
                glGetProgramiv(
                    self.program,
                    GlProgramParam::InfoLogLength.into(),
                    &mut length,
                );
                let mut msg: Vec<u8> = Vec::with_capacity(length as usize);
                glGetProgramInfoLog(
                    self.program,
                    length,
                    &mut length,
                    msg.as_mut_ptr() as *mut i8,
                );
                msg.set_len(length as usize);
                // TODO: replace prog name with prog shaders names
                Err(Error::compile_program("", String::from_utf8(msg).unwrap()))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_uniform_loc(&self, name: &str) -> Result<i32> {
        unsafe {
            let name = CString::new(name).unwrap();
            let loc = glGetUniformLocation(self.program, name.as_ptr());
            Error::gl("glGetUniformLocation")?;
            Ok(loc)
        }
    }
}

pub trait Uniform<T> {
    fn set_uniform(&self, name: &str, data: T) -> Result<()>;
}

impl Uniform<f32> for GlProgram {
    fn set_uniform(&self, name: &str, data: f32) -> Result<()> {
        unsafe {
            let loc = self.get_uniform_loc(name)?;
            glUniform1f(loc, data);
            Error::gl("glUniform1f")
        }
    }
}

impl Uniform<&[f32; 2]> for GlProgram {
    fn set_uniform(&self, name: &str, data: &[f32; 2]) -> Result<()> {
        unsafe {
            let loc = self.get_uniform_loc(name)?;
            glUniform2f(loc, data[0], data[1]);
            Error::gl("glUniform1f")
        }
    }
}

impl Uniform<&[f32; 3]> for GlProgram {
    fn set_uniform(&self, name: &str, data: &[f32; 3]) -> Result<()> {
        unsafe {
            let loc = self.get_uniform_loc(name)?;
            glUniform3f(loc, data[0], data[1], data[2]);
            Error::gl("glUniform1f")
        }
    }
}

impl Uniform<&[f32; 4]> for GlProgram {
    fn set_uniform(&self, name: &str, data: &[f32; 4]) -> Result<()> {
        unsafe {
            let loc = self.get_uniform_loc(name)?;
            glUniform4f(loc, data[0], data[1], data[2], data[3]);
            Error::gl("glUniform1f")
        }
    }
}
