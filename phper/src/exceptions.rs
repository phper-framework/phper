// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{classes::ClassEntry, errors::Throwable, sys::*};
use derive_more::Constructor;

#[inline]
pub fn throwable_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_throwable) }
}

#[inline]
pub fn exception_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_exception) }
}

/// Mainly info for php Exception.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("Uncaught {class_name}: {message} in {file}:{line}")]
pub struct Exception {
    class_name: String,
    code: i64,
    message: String,
    file: String,
    line: i64,
}

impl Exception {
    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn line(&self) -> i64 {
        self.line
    }
}

impl Throwable for Exception {
    fn class_entry(&self) -> &ClassEntry {
        ClassEntry::from_globals(&self.class_name).unwrap()
    }

    fn code(&self) -> i64 {
        self.code
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}
