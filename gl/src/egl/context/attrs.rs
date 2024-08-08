use crate::{attr_list, egl::types::*};

pub const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;
pub const EGL_CONTEXT_MAJOR_VERSION: EGLint = 0x3098;
pub const EGL_CONTEXT_MINOR_VERSION: EGLint = 0x30FB;

attr_list!(
    pub struct EglCtxAttrs {
        client_version: i32 = EGL_CONTEXT_CLIENT_VERSION,
        major_version: i32 = EGL_CONTEXT_MAJOR_VERSION,
        minor_version: i32 = EGL_CONTEXT_MINOR_VERSION,
    }
);
