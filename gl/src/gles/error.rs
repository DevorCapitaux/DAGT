use super::types::*;
use crate::raw_error;
use std::{error::Error, fmt::Display};

pub const GL_NO_ERROR: GLenum = 0x0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;

raw_error!(
    pub enum GLError {
        /// No error has been recorded.
        NoError = GL_NO_ERROR,
        /// An unacceptable value is specified for an enumerated argument.
        InvalidEnum = GL_INVALID_ENUM,
        /// A numeric argument is out of range.
        InvalidValue = GL_INVALID_VALUE,
        /// The specified operation is not allowed in the current state.
        InvalidOperation = GL_INVALID_OPERATION,
        /// The framebuffer object is not complete.
        InvalidFrameBufferOperation = GL_INVALID_FRAMEBUFFER_OPERATION,
        /// There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded.
        OutOfMemory = GL_OUT_OF_MEMORY,
    }
);
