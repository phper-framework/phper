// Copyright (c) 2019 jmjoy
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
use std::{convert::TryInto, ptr::null};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error = E_ERROR,
    Warning = E_WARNING,
    Notice = E_NOTICE,
    Deprecated = E_DEPRECATED,
}

pub fn log(level: LogLevel, message: impl ToString) {
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

pub fn echo(message: impl ToString) {
    let message = ensure_end_with_zero(message);
    unsafe {
        zend_write.expect("function zend_write can't be null")(
            message.as_ptr().cast(),
            (message.len() - 1).try_into().unwrap(),
        );
    }
}
