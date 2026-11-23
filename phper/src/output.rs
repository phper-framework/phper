//! Logs and echo facilities.

use crate::{sys::*, utils::ensure_end_with_zero};

/// Log level.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    /// Error level.
    Error = E_ERROR,
    /// Warning level.
    Warning = E_WARNING,
    /// Notice level.
    Notice = E_NOTICE,
    /// Deprecated level.
    Deprecated = E_DEPRECATED,
}

/// log message with level.
pub fn log(level: LogLevel, message: impl Into<String>) {
    let message = ensure_end_with_zero(message);
    unsafe {
        phper_php_error_docref(level as i32, message.as_ptr().cast());
    }
}

/// Just like PHP `echo`.
#[allow(clippy::useless_conversion)]
pub fn echo(message: impl Into<String>) {
    let message = ensure_end_with_zero(message);
    unsafe {
        zend_write.expect("function zend_write can't be null")(
            message.as_ptr().cast(),
            message.as_bytes().len().try_into().unwrap(),
        );
    }
}
