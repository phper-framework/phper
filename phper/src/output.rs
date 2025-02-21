// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Logs and echo facilities.

use crate::{sys::*, utils::ensure_end_with_zero};
use std::ptr::null;

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
        php_error_docref1(
            null(),
            c"".as_ptr().cast(),
            level as i32,
            message.as_ptr().cast(),
        );
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
