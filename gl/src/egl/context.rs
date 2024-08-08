use super::{binds::*, surface::EglSurface, types::*};
use crate::error::{Error, Result};

pub mod attrs;

pub const EGL_NO_CONTEXT: EGLContext = 0x0 as EGLContext;

pub struct EglContext {
    pub(super) display: EGLDisplay,
    pub(super) context: EGLContext,
}

impl EglContext {
    pub fn make_current(&self, draw: &EglSurface, read: &EglSurface) -> Result<()> {
        unsafe {
            match eglMakeCurrent(self.display, draw.surface, read.surface, self.context) {
                EglBoolean::True => Ok(()),
                EglBoolean::False => Err(Error::egl("eglMakeCurrent")),
            }
        }
    }

    pub fn term(&self) -> Result<()> {
        unsafe {
            match eglDestroyContext(self.display, self.context) {
                EglBoolean::True => Ok(()),
                EglBoolean::False => Err(Error::egl("eglDestroyContext")),
            }
        }
    }
}

impl Drop for EglContext {
    fn drop(&mut self) {
        let _ = self.term();
    }
}
