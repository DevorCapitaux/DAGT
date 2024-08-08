use super::types::*;

pub mod attrs;

#[derive(Clone, Copy)]
pub struct EglConfig {
    pub(super) display: EGLDisplay,
    pub(super) config: EGLConfig,
}
