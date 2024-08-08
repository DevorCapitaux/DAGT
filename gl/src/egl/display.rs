use super::{
    binds::*,
    config::{attrs::EglConfAttrs, EglConfig},
    context::{attrs::EglCtxAttrs, EglContext, EGL_NO_CONTEXT},
    surface::{EglSurface, EGL_NO_SURFACE},
    types::{EglBoolean::*, *},
};
use crate::error::{Error, Result};

pub const EGL_NO_DISPLAY: EGLDisplay = 0x0 as EGLDisplay;
pub const EGL_DEFAULT_DISPLAY: NativeDisplayType = 0x0 as NativeDisplayType;

pub struct EglDisplay {
    display: EGLDisplay,
}

impl EglDisplay {
    pub fn get(native_display: NativeDisplayType) -> Option<EglDisplay> {
        unsafe {
            let display = eglGetDisplay(native_display);
            match display {
                EGL_NO_DISPLAY => None,
                _ => Some(EglDisplay { display }),
            }
        }
    }

    pub fn get_default() -> Option<EglDisplay> {
        EglDisplay::get(EGL_DEFAULT_DISPLAY)
    }

    pub fn get_current() -> Option<EglDisplay> {
        unsafe {
            let display = eglGetCurrentDisplay();
            match display {
                EGL_NO_DISPLAY => None,
                _ => Some(EglDisplay { display }),
            }
        }
    }

    pub fn init(&self) -> Result<(i32, i32)> {
        unsafe {
            let mut major = 0;
            let mut minor = 0;
            match eglInitialize(self.display, &mut major, &mut minor) {
                True => Ok((major, minor)),
                False => Err(Error::egl("eglInitialize")),
            }
        }
    }

    pub fn get_confs_num(&self, attrs: Option<&EglConfAttrs>) -> Result<i32> {
        unsafe {
            let mut func_name = "eglChooseConfig";
            let mut confs_num = 0;
            match attrs
                .map(|attrs| {
                    eglChooseConfig(
                        self.display,
                        attrs.as_ptr(),
                        std::ptr::null_mut(),
                        0,
                        &mut confs_num,
                    )
                })
                .unwrap_or_else(|| {
                    func_name = "eglChooseConfig";
                    eglGetConfigs(self.display, std::ptr::null_mut(), 0, &mut confs_num)
                }) {
                EglBoolean::True => Ok(confs_num),
                EglBoolean::False => Err(Error::egl(func_name)),
            }
        }
    }

    pub fn get_confs(&self, attrs: Option<&EglConfAttrs>) -> Result<Vec<EglConfig>> {
        unsafe {
            let mut func_name = "eglChooseConfig";
            let mut confs_num = self.get_confs_num(attrs).unwrap();
            let mut confs: Vec<EGLConfig> = Vec::with_capacity(confs_num as usize);
            match attrs
                .map(|attrs| {
                    eglChooseConfig(
                        self.display,
                        attrs.as_ptr(),
                        confs.as_mut_ptr(),
                        confs_num,
                        &mut confs_num,
                    )
                })
                .unwrap_or_else(|| {
                    func_name = "eglChooseConfig";
                    eglGetConfigs(self.display, confs.as_mut_ptr(), confs_num, &mut confs_num)
                }) {
                EglBoolean::True => {
                    confs.set_len(confs_num as usize);
                    Ok(confs
                        .iter_mut()
                        .map(|config| EglConfig {
                            display: self.display,
                            config: *config,
                        })
                        .collect())
                }
                EglBoolean::False => Err(Error::egl(func_name)),
            }
        }
    }

    pub fn get_conf_first(&self, attrs: Option<&EglConfAttrs>) -> Option<EglConfig> {
        match self.get_confs(attrs) {
            Ok(v) => v.first().copied(),
            Err(_) => None,
        }
    }

    pub fn create_window_surface(
        &self,
        config: &EglConfig,
        native_window: NativeWindowType,
    ) -> Result<EglSurface> {
        unsafe {
            let surface = eglCreateWindowSurface(
                self.display,
                config.config,
                native_window,
                std::ptr::null(),
            );
            if surface == EGL_NO_SURFACE {
                Err(Error::egl("eglCreateWindowSurface"))
            } else {
                Ok(EglSurface {
                    display: self.display,
                    surface,
                })
            }
        }
    }

    pub fn create_context(
        &self,
        config: &EglConfig,
        share_ctx: Option<&EglContext>,
        attrs: Option<&EglCtxAttrs>,
    ) -> Result<EglContext> {
        unsafe {
            let share_ctx = match share_ctx {
                Some(ctx) => ctx.context,
                None => std::ptr::null_mut(),
            };
            let attrs = match attrs {
                Some(list) => list.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let context = eglCreateContext(self.display, config.config, share_ctx, attrs);
            if context == EGL_NO_CONTEXT {
                Err(Error::egl("eglCreateContext"))
            } else {
                Ok(EglContext {
                    display: self.display,
                    context,
                })
            }
        }
    }

    pub fn term(&self) -> Result<()> {
        unsafe {
            match eglTerminate(self.display) {
                EglBoolean::True => Ok(()),
                EglBoolean::False => Err(Error::egl("eglTerminate")),
            }
        }
    }
}

impl Drop for EglDisplay {
    fn drop(&mut self) {
        let _ = self.term();
    }
}
