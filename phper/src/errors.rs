//! The errors for crate and php.

use crate as phper;
use crate::{
    classes::{ClassEntry, StatelessClassEntry},
    exceptions::Exception,
    sys::*,
    Error::Other,
};
use anyhow::anyhow;
use derive_more::Constructor;
use std::{convert::Infallible, error, ffi::FromBytesWithNulError, io, str::Utf8Error};

const ARGUMENT_COUNT_ERROR_CLASS: &'static str = if PHP_VERSION_ID >= 70100 {
    "ArgumentCountError"
} else {
    "TypeError"
};

/// PHP Throwable, can cause throwing an exception when setting to [crate::values::Val].
pub trait Throwable: error::Error {
    fn class_entry(&self) -> &StatelessClassEntry;

    fn code(&self) -> i64 {
        0
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

impl Throwable for Infallible {
    fn class_entry(&self) -> &StatelessClassEntry {
        unreachable!()
    }
}

/// Type of [std::result::Result]<T, [crate::Error]>.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Crate level Error, which also can become an exception in php.
///
/// As a php exception, will throw `ErrorException` when the item not implement [Throwable].
#[derive(thiserror::Error, crate::Throwable, Debug)]
#[throwable_class("ErrorException")]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    #[throwable(transparent)]
    Throw(#[from] Exception),

    #[error(transparent)]
    #[throwable(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    #[throwable(transparent)]
    ClassNotFound(#[from] ClassNotFoundError),

    #[error(transparent)]
    #[throwable(transparent)]
    ArgumentCount(#[from] ArgumentCountError),

    #[error(transparent)]
    #[throwable(transparent)]
    StateType(#[from] StateTypeError),

    #[error(transparent)]
    #[throwable(transparent)]
    CallFunction(#[from] CallFunctionError),

    #[error(transparent)]
    #[throwable(transparent)]
    CallMethod(#[from] CallMethodError),

    #[error(transparent)]
    #[throwable(transparent)]
    InitializeObject(#[from] InitializeObjectError),

    #[error(transparent)]
    #[throwable(transparent)]
    NotRefCountedType(#[from] NotRefCountedTypeError),
}

impl Error {
    /// An essy way to cause an [anyhow::Error].
    pub fn other(message: impl ToString) -> Self {
        let message = message.to_string();
        Other(anyhow!(message))
    }
}

#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("type error: {message}")]
#[throwable_class("TypeError")]
pub struct TypeError {
    message: String,
}

#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("Class '{class_name}' not found")]
#[throwable_class("Error")]
pub struct ClassNotFoundError {
    class_name: String,
}

#[derive(Debug, thiserror::Error, crate::Throwable)]
#[error(
    "Actual State type in generic type parameter isn't the state type registered in the class, \
please confirm the real state type, or use StatelessClassEntry"
)]
#[throwable_class("Error")]
pub struct StateTypeError;

#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("{function_name}(): expects at least {expect_count} parameter(s), {given_count} given")]
#[throwable_class(ARGUMENT_COUNT_ERROR_CLASS)]
pub struct ArgumentCountError {
    function_name: String,
    expect_count: usize,
    given_count: usize,
}

/// TODO Merge CallMethodError.
#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("Invalid call to {fn_name}")]
#[throwable_class("BadFunctionCallException")]
pub struct CallFunctionError {
    fn_name: String,
}

impl CallFunctionError {
    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }
}

#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("Invalid call to {class_name}::{method_name}")]
#[throwable_class("BadMethodCallException")]
pub struct CallMethodError {
    class_name: String,
    method_name: String,
}

#[derive(Debug, thiserror::Error, crate::Throwable, Constructor)]
#[error("Cannot instantiate class {class_name}")]
#[throwable_class("Error")]
pub struct InitializeObjectError {
    class_name: String,
}

#[derive(Debug, thiserror::Error, crate::Throwable)]
#[error("the type is not refcounted")]
#[throwable_class("TypeError")]
pub struct NotRefCountedTypeError;
