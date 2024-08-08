use crate::{attr_bitstruct, attr_list, egl::types::*};

pub const EGL_BUFFER_SIZE: EGLint = 0x3020;
pub const EGL_ALPHA_SIZE: EGLint = 0x3021;
pub const EGL_BLUE_SIZE: EGLint = 0x3022;
pub const EGL_GREEN_SIZE: EGLint = 0x3023;
pub const EGL_RED_SIZE: EGLint = 0x3024;
pub const EGL_DEPTH_SIZE: EGLint = 0x3025;
pub const EGL_STENCIL_SIZE: EGLint = 0x3026;
pub const EGL_SURFACE_TYPE: EGLint = 0x3033;

attr_list!(
    pub struct EglConfAttrs {
        buffer_size: i32 = EGL_BUFFER_SIZE,
        red_size: i32 = EGL_RED_SIZE,
        green_size: i32 = EGL_GREEN_SIZE,
        blue_size: i32 = EGL_BLUE_SIZE,
        alpha_size: i32 = EGL_ALPHA_SIZE,
        depth_size: i32 = EGL_DEPTH_SIZE,
        stencil_size: i32 = EGL_STENCIL_SIZE,
        surface_type: EglSurfaceType = EGL_SURFACE_TYPE,
    }
);

pub const EGL_PBUFFER_BIT: EGLint = 0x01;
pub const EGL_PIXMAP_BIT: EGLint = 0x02;
pub const EGL_WINDOW_BIT: EGLint = 0x04;

attr_bitstruct!(
    pub bitstruct EglSurfaceType(EGL_WINDOW_BIT) {
        pbuffer = EGL_PBUFFER_BIT,
        pixmap = EGL_PIXMAP_BIT,
        window = EGL_WINDOW_BIT,
    }
);
