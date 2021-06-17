use crate::{classes::StatelessClassEntry, sys::*};
use derive_more::Constructor;

#[inline]
pub fn throwable_class<'a>() -> &'a StatelessClassEntry {
    unsafe { StatelessClassEntry::from_ptr(zend_ce_throwable) }
}

#[inline]
pub fn exception_class<'a>() -> &'a StatelessClassEntry {
    unsafe { StatelessClassEntry::from_ptr(zend_ce_exception) }
}

/// Mainly info for php Exception.
/// TODO Add file and line.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("Uncaught {class_name}: {message} in ?:?")]
pub struct Exception {
    class_name: String,
    code: i64,
    message: String,
    // file: String,
    // line: i64,
}

impl Exception {
    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    pub fn code(&self) -> i64 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
