// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! The errors for crate and php.

use crate::{classes::ClassEntry, objects::ZObject, sys::*, types::TypeInfo, values::ZVal};
use derive_more::Constructor;
use phper_alloc::ToRefOwned;
use std::{
    cell::RefCell,
    convert::Infallible,
    error,
    ffi::FromBytesWithNulError,
    fmt::{self, Debug, Display},
    io,
    marker::PhantomData,
    mem::{ManuallyDrop, replace},
    ops::{Deref, DerefMut},
    ptr::null_mut,
    result,
    str::Utf8Error,
};

/// Predefined interface `Throwable`.
#[inline]
pub fn throwable_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_throwable) }
}

/// Predefined class `Exception`.
#[inline]
pub fn exception_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_exception) }
}

/// Predefined class `Error`.
#[inline]
pub fn error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_error) }
}

/// Predefined class `ErrorException`.
#[inline]
pub fn error_exception_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_error_exception) }
}

/// Predefined class `TypeError`.
#[inline]
pub fn type_error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_type_error) }
}

/// Predefined class `ArgumentCountError` (>= PHP 7.1.0).
#[cfg(not(all(phper_major_version = "7", phper_minor_version = "0")))]
#[inline]
pub fn argument_count_error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_argument_count_error) }
}

/// Predefined class `ArithmeticError`.
#[inline]
pub fn arithmetic_error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_arithmetic_error) }
}

/// Predefined class `ParseError`.
#[inline]
pub fn parse_error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_parse_error) }
}

/// Predefined class `DivisionByZeroError`.
#[inline]
pub fn division_by_zero_error<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_division_by_zero_error) }
}

/// PHP Throwable, can cause throwing an exception when setting to
/// [crate::values::ZVal].
pub trait Throwable: error::Error {
    /// Gets the class reference, implemented PHP `Throwable`.
    fn get_class(&self) -> &ClassEntry;

    /// Gets the exception code.
    #[inline]
    fn get_code(&self) -> Option<i64> {
        Some(0)
    }

    /// Gets the message.
    #[inline]
    fn get_message(&self) -> Option<String> {
        Some(self.to_string())
    }

    /// Create Exception object.
    ///
    /// By default, the Exception is instance of `get_class()`, the code is
    /// `get_code` and message is `get_message`;
    fn to_object(&mut self) -> result::Result<ZObject, Box<dyn Throwable>> {
        let mut object =
            ZObject::new(self.get_class(), []).map_err(|e| Box::new(e) as Box<dyn Throwable>)?;
        if let Some(code) = self.get_code() {
            object.set_property("code", code);
        }
        if let Some(message) = self.get_message() {
            object.set_property("message", message);
        }
        Ok(object)
    }
}

impl<T: Throwable> Throwable for Box<T> {
    fn get_class(&self) -> &ClassEntry {
        Throwable::get_class(self.deref())
    }

    fn get_code(&self) -> Option<i64> {
        Throwable::get_code(self.deref())
    }

    fn get_message(&self) -> Option<String> {
        Throwable::get_message(self.deref())
    }

    fn to_object(&mut self) -> result::Result<ZObject, Box<dyn Throwable>> {
        Throwable::to_object(self.deref_mut())
    }
}

impl Throwable for dyn error::Error {
    fn get_class(&self) -> &ClassEntry {
        error_exception_class()
    }
}

impl Throwable for Infallible {
    fn get_class(&self) -> &ClassEntry {
        match *self {}
    }

    fn get_code(&self) -> Option<i64> {
        match *self {}
    }

    fn get_message(&self) -> Option<String> {
        match *self {}
    }

    fn to_object(&mut self) -> result::Result<ZObject, Box<dyn Throwable>> {
        match *self {}
    }
}

/// Type of [Result]<T, [crate::Error]>.
///
/// [Result]: std::result::Result
pub type Result<T> = result::Result<T, self::Error>;

/// Crate level Error, which also can become an exception in php.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The error type for I/O operations.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Errors which can occur when attempting to interpret a sequence of [`u8`]
    /// as a string.
    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    /// An error indicating that a nul byte was not in the expected position.
    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    /// Owned boxed dynamic error.
    #[error(transparent)]
    Boxed(#[from] Box<dyn error::Error>),

    /// Owned exception object.
    #[error(transparent)]
    Throw(#[from] ThrowObject),

    /// Class not found, get the class by name failed, etc.
    #[error(transparent)]
    ClassNotFound(#[from] ClassNotFoundError),

    /// Throw when actual arguments count is not greater than expect in calling
    /// functions.
    #[error(transparent)]
    ArgumentCount(#[from] ArgumentCountError),

    /// Failed to initialize object.
    #[error(transparent)]
    InitializeObject(#[from] InitializeObjectError),

    /// Expect type is not the actual type.
    #[error(transparent)]
    ExpectType(#[from] ExpectTypeError),

    /// Failed when the object isn't implement PHP `Throwable`.
    #[error(transparent)]
    NotImplementThrowable(#[from] NotImplementThrowableError),
}

impl Error {
    /// Wrap the dynamic error into `Boxed`.
    pub fn boxed(e: impl Into<Box<dyn error::Error>> + 'static) -> Self {
        Self::Boxed(e.into())
    }

    /// Transfer [Throwable] item to exception object internally, and wrap it
    /// into `Throw`.
    pub fn throw(t: impl Throwable) -> Self {
        let obj = ThrowObject::from_throwable(t);
        Self::Throw(obj)
    }
}

impl Throwable for Error {
    #[inline]
    fn get_class(&self) -> &ClassEntry {
        match self {
            Error::Io(e) => Throwable::get_class(e as &dyn error::Error),
            Error::Utf8(e) => Throwable::get_class(e as &dyn error::Error),
            Error::FromBytesWithNul(e) => Throwable::get_class(e as &dyn error::Error),
            Error::Boxed(e) => Throwable::get_class(e.deref()),
            Error::Throw(e) => Throwable::get_class(e),
            Error::ClassNotFound(e) => Throwable::get_class(e),
            Error::ArgumentCount(e) => Throwable::get_class(e),
            Error::InitializeObject(e) => Throwable::get_class(e),
            Error::ExpectType(e) => Throwable::get_class(e),
            Error::NotImplementThrowable(e) => Throwable::get_class(e),
        }
    }

    #[inline]
    fn get_code(&self) -> Option<i64> {
        match self {
            Error::Io(e) => Throwable::get_code(e as &dyn error::Error),
            Error::Utf8(e) => Throwable::get_code(e as &dyn error::Error),
            Error::FromBytesWithNul(e) => Throwable::get_code(e as &dyn error::Error),
            Error::Boxed(e) => Throwable::get_code(e.deref()),
            Error::Throw(e) => Throwable::get_code(e),
            Error::ClassNotFound(e) => Throwable::get_code(e),
            Error::ArgumentCount(e) => Throwable::get_code(e),
            Error::InitializeObject(e) => Throwable::get_code(e),
            Error::ExpectType(e) => Throwable::get_code(e),
            Error::NotImplementThrowable(e) => Throwable::get_code(e),
        }
    }

    fn get_message(&self) -> Option<String> {
        match self {
            Error::Io(e) => Throwable::get_message(e as &dyn error::Error),
            Error::Utf8(e) => Throwable::get_message(e as &dyn error::Error),
            Error::FromBytesWithNul(e) => Throwable::get_message(e as &dyn error::Error),
            Error::Boxed(e) => Throwable::get_message(e.deref()),
            Error::Throw(e) => Throwable::get_message(e),
            Error::ClassNotFound(e) => Throwable::get_message(e),
            Error::ArgumentCount(e) => Throwable::get_message(e),
            Error::InitializeObject(e) => Throwable::get_message(e),
            Error::ExpectType(e) => Throwable::get_message(e),
            Error::NotImplementThrowable(e) => Throwable::get_message(e),
        }
    }

    fn to_object(&mut self) -> result::Result<ZObject, Box<dyn Throwable>> {
        match self {
            Error::Io(e) => Throwable::to_object(e as &mut dyn error::Error),
            Error::Utf8(e) => Throwable::to_object(e as &mut dyn error::Error),
            Error::FromBytesWithNul(e) => Throwable::to_object(e as &mut dyn error::Error),
            Error::Boxed(e) => Throwable::to_object(e.deref_mut()),
            Error::Throw(e) => Throwable::to_object(e),
            Error::ClassNotFound(e) => Throwable::to_object(e),
            Error::ArgumentCount(e) => Throwable::to_object(e),
            Error::InitializeObject(e) => Throwable::to_object(e),
            Error::ExpectType(e) => Throwable::to_object(e),
            Error::NotImplementThrowable(e) => Throwable::to_object(e),
        }
    }
}

/// Wrapper of Throwable object.
#[derive(Debug)]
pub struct ThrowObject(ZObject);

impl ThrowObject {
    /// Construct from Throwable object.
    ///
    /// Failed if the object is not instance of php `Throwable`.
    pub fn new(obj: ZObject) -> result::Result<Self, NotImplementThrowableError> {
        if !obj.get_class().is_instance_of(throwable_class()) {
            return Err(NotImplementThrowableError);
        }
        Ok(Self(obj))
    }

    /// Construct from Throwable.
    #[inline]
    pub fn from_throwable(mut t: impl Throwable) -> Self {
        Self::from_result(t.to_object())
    }

    /// Construct from dynamic Error.
    #[inline]
    pub fn from_error(mut e: impl error::Error + 'static) -> Self {
        let e = &mut e as &mut dyn error::Error;
        Self::from_result(Throwable::to_object(e))
    }

    fn from_result(mut result: result::Result<ZObject, Box<dyn Throwable>>) -> Self {
        let mut i = 0;

        let obj = loop {
            match result {
                Ok(o) => break o,
                Err(mut e) => {
                    if i > 50 {
                        panic!("recursion limit reached");
                    }
                    result = e.to_object();
                    i += 1;
                }
            }
        };

        Self::new(obj).unwrap()
    }

    /// Consumes the `ThrowObject`, returning the wrapped object.
    #[inline]
    pub fn into_inner(self) -> ZObject {
        self.0
    }

    fn inner_get_code(&self) -> i64 {
        self.0
            .get_property("code")
            .as_long()
            .expect("code isn't long")
    }

    fn inner_get_message(&self) -> String {
        self.0
            .get_property("message")
            .as_z_str()
            .expect("message isn't string")
            .to_str()
            .map(ToOwned::to_owned)
            .unwrap_or_default()
    }
}

impl Display for ThrowObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner_get_message(), f)
    }
}

impl error::Error for ThrowObject {}

impl Throwable for ThrowObject {
    #[inline]
    fn get_class(&self) -> &ClassEntry {
        self.0.get_class()
    }

    #[inline]
    fn get_code(&self) -> Option<i64> {
        Some(self.inner_get_code())
    }

    #[inline]
    fn get_message(&self) -> Option<String> {
        Some(self.inner_get_message())
    }

    #[inline]
    fn to_object(&mut self) -> result::Result<ZObject, Box<dyn Throwable>> {
        Ok(self.0.to_ref_owned())
    }
}

/// Expect type is not the actual type.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("type error: must be of type {expect_type}, {actual_type} given")]
pub struct ExpectTypeError {
    expect_type: TypeInfo,
    actual_type: TypeInfo,
}

impl Throwable for ExpectTypeError {
    #[inline]
    fn get_class(&self) -> &ClassEntry {
        type_error_class()
    }
}

/// Class not found, get the class by name failed, etc.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("Class '{class_name}' not found")]
pub struct ClassNotFoundError {
    class_name: String,
}

impl Throwable for ClassNotFoundError {
    fn get_class(&self) -> &ClassEntry {
        error_class()
    }
}

/// Throw when actual arguments count is not greater than expect in calling
/// functions.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("{function_name}(): expects at least {expect_count} parameter(s), {given_count} given")]
pub struct ArgumentCountError {
    function_name: String,
    expect_count: usize,
    given_count: usize,
}

impl Throwable for ArgumentCountError {
    fn get_class(&self) -> &ClassEntry {
        #[cfg(not(all(phper_major_version = "7", phper_minor_version = "0")))]
        {
            argument_count_error_class()
        }

        #[cfg(all(phper_major_version = "7", phper_minor_version = "0"))]
        {
            type_error_class()
        }
    }
}

/// Failed to initialize object.
#[derive(Debug, thiserror::Error, Constructor)]
#[error("Cannot instantiate class {class_name}")]
pub struct InitializeObjectError {
    class_name: String,
}

impl Throwable for InitializeObjectError {
    fn get_class(&self) -> &ClassEntry {
        error_class()
    }
}

/// Failed when the object isn't implement PHP `Throwable`.
#[derive(Debug, thiserror::Error)]
#[error("Cannot throw objects that do not implement Throwable")]
pub struct NotImplementThrowableError;

impl Throwable for NotImplementThrowableError {
    fn get_class(&self) -> &ClassEntry {
        type_error_class()
    }
}

/// Guarder for preventing the thrown exception from being overwritten.
///
/// Normally, you don't need to use `ExceptionGuard`, unless before you call the
/// unsafe raw php function, like `call_user_function`.
///
/// Can be used nested.
pub struct ExceptionGuard(PhantomData<*mut ()>);

thread_local! {
    static EXCEPTION_STACK: RefCell<Vec<*mut zend_object>> = Default::default();
}

impl Default for ExceptionGuard {
    fn default() -> Self {
        EXCEPTION_STACK.with(|stack| unsafe {
            #[allow(static_mut_refs)]
            stack
                .borrow_mut()
                .push(replace(&mut eg!(exception), null_mut()));
        });
        Self(PhantomData)
    }
}

impl Drop for ExceptionGuard {
    fn drop(&mut self) {
        EXCEPTION_STACK.with(|stack| unsafe {
            eg!(exception) = stack.borrow_mut().pop().expect("exception stack is empty");
        });
    }
}

/// # Safety
///
/// You should always return the `Result<ZVal, impl Throwable>` in the handler,
/// rather than use this function.
pub unsafe fn throw(e: impl Throwable) {
    unsafe {
        let obj = ThrowObject::from_throwable(e).into_inner();
        let mut val = ManuallyDrop::new(ZVal::from(obj));
        zend_throw_exception_object(val.as_mut_ptr());
    }
}

/// Equivalent to `Ok::<_, phper::Error>(value)`.
#[inline]
pub fn ok<T>(t: T) -> Result<T> {
    Ok(t)
}
