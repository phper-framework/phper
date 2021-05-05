use crate::{
    classes::{ClassEntity, ClassEntry},
    modules::read_global_module,
    Error::{ClassNotFound, Other},
};
use anyhow::anyhow;
use std::{error, ffi::FromBytesWithNulError, io, str::Utf8Error};

pub(crate) const EXCEPTION_CLASS_NAME: &'static str = "Phper\\OtherException";

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

/// Type of [std::result::Result]<T, [crate::Error]>.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Crate level Error, which also can become an exception in php.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    #[error(transparent)]
    TypeError(#[from] TypeError),

    #[error(transparent)]
    ClassNotFound(#[from] ClassNotFoundError),

    #[error(transparent)]
    ArgumentCountError(#[from] ArgumentCountError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// An essy way to cause an anyhow::Error.
    pub fn other(message: impl ToString) -> Self {
        let message = message.to_string();
        Other(anyhow!(message))
    }
}

// TODO Add message() implement.
impl Throwable for Error {
    fn class_entry(&self) -> &ClassEntry {
        match self {
            Self::TypeError(e) => e.class_entry(),
            Self::ClassNotFound(e) => e.class_entry(),
            Self::ArgumentCountError(e) => e.class_entry(),
            _ => ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap(),
        }
    }

    fn code(&self) -> u64 {
        match self {
            Self::TypeError(e) => e.code(),
            Self::ClassNotFound(e) => e.code(),
            Self::ArgumentCountError(e) => e.code(),
            _ => 0,
        }
    }
}

/// PHP type error.
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
        ClassEntry::from_globals("ArgumentCountError").unwrap()
    }
}
