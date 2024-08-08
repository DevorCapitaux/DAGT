use super::types::*;

#[link(name = "EGL")]
extern "C" {
    pub fn eglGetError() -> EGLint;

    pub fn eglGetDisplay(native_display: NativeDisplayType) -> EGLDisplay;
    pub fn eglGetCurrentDisplay() -> EGLDisplay;
    pub fn eglInitialize(display: EGLDisplay, major: *mut EGLint, minor: *mut EGLint)
        -> EglBoolean;
    pub fn eglTerminate(display: EGLDisplay) -> EglBoolean;

    pub fn eglGetConfigs(
        display: EGLDisplay,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EglBoolean;
    pub fn eglChooseConfig(
        display: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EglBoolean;

    pub fn eglCreateWindowSurface(
        display: EGLDisplay,
        config: EGLConfig,
        native_window: NativeWindowType,
        attrib_list: *const EGLint,
    ) -> EGLSurface;

    pub fn eglCreateContext(
        display: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext;
    pub fn eglMakeCurrent(
        display: EGLDisplay,
        draw: EGLSurface,
        read: EGLSurface,
        context: EGLContext,
    ) -> EglBoolean;
    pub fn eglDestroyContext(display: EGLDisplay, context: EGLContext) -> EglBoolean;

    pub fn eglSwapBuffers(display: EGLDisplay, surface: EGLSurface) -> EglBoolean;
    pub fn eglDestroySurface(display: EGLDisplay, surface: EGLSurface) -> EglBoolean;
}
