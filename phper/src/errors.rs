//! The errors for crate and php.

use crate::{classes::ClassEntry, sys::*, Error::Other};
use anyhow::anyhow;
use std::{convert::Infallible, error, ffi::FromBytesWithNulError, io, str::Utf8Error};

/// PHP Throwable, can cause throwing an exception when setting to [crate::values::Val].
pub trait Throwable: error::Error {
    fn class_entry(&self) -> &ClassEntry;

    fn code(&self) -> u64 {
        0
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl Throwable for Infallible {
    fn class_entry(&self) -> &ClassEntry {
        unreachable!()
    }
}

/// Type of [std::result::Result]<T, [crate::Error]>.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Crate level Error, which also can become an exception in php.
///
/// As a php exception, will throw `ErrorException` when the item not implement [Throwable].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    ClassNotFound(#[from] ClassNotFoundError),

    #[error(transparent)]
    ArgumentCount(#[from] ArgumentCountError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// An essy way to cause an [anyhow::Error].
    pub fn other(message: impl ToString) -> Self {
        let message = message.to_string();
        Other(anyhow!(message))
    }
}

// TODO Add message() implement.
impl Throwable for Error {
    fn class_entry(&self) -> &ClassEntry {
        match self {
            Self::Type(e) => e.class_entry(),
            Self::ClassNotFound(e) => e.class_entry(),
            Self::ArgumentCount(e) => e.class_entry(),
            _ => ClassEntry::from_globals("ErrorException").unwrap(),
        }
    }

    fn code(&self) -> u64 {
        match self {
            Self::Type(e) => e.code(),
            Self::ClassNotFound(e) => e.code(),
            Self::ArgumentCount(e) => e.code(),
            _ => 0,
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("type error: {message}")]
pub struct TypeError {
    message: String,
}

impl TypeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Throwable for TypeError {
    fn class_entry(&self) -> &ClassEntry {
        ClassEntry::from_globals("TypeError").unwrap()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Class '{class_name}' not found")]
pub struct ClassNotFoundError {
    class_name: String,
}

impl ClassNotFoundError {
    pub fn new(class_name: String) -> Self {
        Self { class_name }
    }
}

impl Throwable for ClassNotFoundError {
    fn class_entry(&self) -> &ClassEntry {
        ClassEntry::from_globals("Error").unwrap()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{function_name}(): expects at least {expect_count} parameter(s), {given_count} given")]
pub struct ArgumentCountError {
    function_name: String,
    expect_count: usize,
    given_count: usize,
}

impl ArgumentCountError {
    pub fn new(function_name: String, expect_count: usize, given_count: usize) -> Self {
        Self {
            function_name,
            expect_count,
            given_count,
        }
    }
}

impl Throwable for ArgumentCountError {
    fn class_entry(&self) -> &ClassEntry {
        let class_name = if PHP_VERSION_ID >= 70100 {
            "ArgumentCountError"
        } else {
            "TypeError"
        };
        ClassEntry::from_globals(class_name).unwrap()
    }
}
