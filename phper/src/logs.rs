use crate::{sys::*, utils::ensure_end_with_zero};
use std::ptr::null;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum Level {
    Error = E_ERROR,
    Warning = E_WARNING,
    Notice = E_NOTICE,
    Deprecated = E_DEPRECATED,
}

pub fn log(level: Level, message: impl ToString) {
    let message = ensure_end_with_zero(message);
    unsafe {
        php_error_docref1(
            null(),
            "\0".as_ptr().cast(),
            level as i32,
            message.as_ptr().cast(),
        );
    }
}
