use std::ffi::c_void;

pub type EGLint = i32;
pub type EGLenum = u32;

pub type NativeDisplayType = *mut c_void;
pub type NativeWindowType = *mut c_void;

pub type EGLDisplay = *mut c_void;
pub type EGLSurface = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLContext = *mut c_void;

pub const EGL_TRUE: EGLint = 1;
pub const EGL_FALSE: EGLint = 0;
pub const EGL_NONE: EGLint = 0x3038;

#[repr(u32)]
pub enum EglBoolean {
    True = EGL_TRUE as u32,
    False = EGL_FALSE as u32,
}
