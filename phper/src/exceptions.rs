use crate::{classes::StatelessClassEntry, errors::Throwable, sys::*};
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
    fn class_entry(&self) -> &StatelessClassEntry {
        StatelessClassEntry::from_globals(&self.class_name).unwrap()
    }

    fn code(&self) -> i64 {
        self.code
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}
