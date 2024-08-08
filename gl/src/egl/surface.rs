use super::{binds::*, types::*};
use crate::error::{Error, Result};

pub const EGL_NO_SURFACE: EGLSurface = 0x0 as EGLSurface;

pub struct EglSurface {
    pub(super) display: EGLDisplay,
    pub(super) surface: EGLSurface,
}

impl EglSurface {
    pub fn swap(&self) -> Result<()> {
        unsafe {
            match eglSwapBuffers(self.display, self.surface) {
                EglBoolean::True => Ok(()),
                EglBoolean::False => Err(Error::egl("eglSwapBuffers")),
            }
        }
    }

    pub fn term(&self) -> Result<()> {
        unsafe {
            match eglDestroySurface(self.display, self.surface) {
                EglBoolean::True => Ok(()),
                EglBoolean::False => Err(Error::egl("eglDestroySurface")),
            }
        }
    }
}
