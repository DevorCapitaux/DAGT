use super::types::EGLint;
use crate::raw_error;
use std::{error::Error, fmt::Display};

pub const EGL_SUCCESS: EGLint = 0x3000;
pub const EGL_NOT_INITIALIZED: EGLint = 0x3001;
pub const EGL_BAD_ACCESS: EGLint = 0x3002;
pub const EGL_BAD_ALLOC: EGLint = 0x3003;
pub const EGL_BAD_ATTRIBUTE: EGLint = 0x3004;
pub const EGL_BAD_CONFIG: EGLint = 0x3005;
pub const EGL_BAD_CONTEXT: EGLint = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE: EGLint = 0x3007;
pub const EGL_BAD_DISPLAY: EGLint = 0x3008;
pub const EGL_BAD_MATCH: EGLint = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP: EGLint = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW: EGLint = 0x300B;
pub const EGL_BAD_PARAMETER: EGLint = 0x300C;
pub const EGL_BAD_SURFACE: EGLint = 0x300D;
pub const EGL_CONTEXT_LOST: EGLint = 0x300E;

raw_error!(
    pub enum EGLError {
        /// The last function succeeded without error.
        EglSuccess = EGL_SUCCESS,
        /// EGL is not initialized, or could not be initialized, for the specified EGL display connection.
        EglNotInitialized = EGL_NOT_INITIALIZED,
        /// EGL cannot access a requested resource (for example a context is bound in another thread).
        EglBadAccess = EGL_BAD_ACCESS,
        /// EGL failed to allocate resources for the requested operation.
        EglBadAlloc = EGL_BAD_ALLOC,
        /// An unrecognized attribute or attribute value was passed in the attribute list.
        EglBadAttribute = EGL_BAD_ATTRIBUTE,
        /// An EGLConfig argument does not name a valid EGL frame buffer configuration.
        EglBadConfig = EGL_BAD_CONFIG,
        /// An EGLContext argument does not name a valid EGL rendering context.
        EglBadContext = EGL_BAD_CONTEXT,
        /// The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.
        EglBadCurrentSurface = EGL_BAD_CURRENT_SURFACE,
        /// An EGLDisplay argument does not name a valid EGL display connection.
        EglBadDisplay = EGL_BAD_DISPLAY,
        /// Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
        EglBadMatch = EGL_BAD_MATCH,
        /// A NativePixmapType argument does not refer to a valid native pixmap.
        EglBadNativePixmap = EGL_BAD_NATIVE_PIXMAP,
        /// A NativeWindowType argument does not refer to a valid native window.
        EglBadNativewindow = EGL_BAD_NATIVE_WINDOW,
        /// One or more argument values are invalid.
        EglBadParameter = EGL_BAD_PARAMETER,
        /// An EGLSurface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.
        EglBadSurface = EGL_BAD_SURFACE,
        /// A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering.
        EglContextLost = EGL_CONTEXT_LOST,
    }
);
