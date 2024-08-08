use crate::{
    egl::{binds::eglGetError, error::EGLError},
    gles::{binds::glGetError, error::GLError},
};
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Egl(EGLError, String),
    Gl(GLError, String),
    Compile(String, String),
}

impl Error {
    pub unsafe fn egl(func: &str) -> Error {
        Error::Egl(eglGetError().try_into().unwrap(), func.to_owned())
    }

    pub unsafe fn gl(func: &str) -> Result<()> {
        match glGetError().try_into().unwrap() {
            GLError::NoError => Ok(()),
            e => Err(Error::Gl(e, func.to_owned())),
        }
    }

    pub unsafe fn gl_err(func: &str) -> Error {
        Error::Gl(glGetError().try_into().unwrap(), func.to_owned())
    }

    pub unsafe fn compile_shader(shader_name: &str, e: String) -> Error {
        Error::Compile(e, format!("shader `{}`", shader_name.to_owned()))
    }

    pub unsafe fn compile_program(program_name: &str, e: String) -> Error {
        Error::Compile(e, format!("program `{}`", program_name.to_owned()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Egl(e, func) => write!(f, "error in `{func}`: {e}"),
            Gl(e, func) => write!(f, "error in `{func}`: {e}"),
            Compile(e, func) => write!(f, "error in `{func}`: {e}"),
        }
    }
}

impl std::error::Error for Error {}
