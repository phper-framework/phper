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

use crate::{classes::ClassEntry, objects::ZObject, sys::*, types::TypeInfo};
use derive_more::Constructor;
use phper_alloc::ToRefOwned;
use std::{
    error,
    ffi::FromBytesWithNulError,
    fmt::{self, Debug, Display},
    io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
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
#[cfg(phper_version_id_gte_70100)]
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

/// Predefined class `CompileError`.
#[inline]
pub fn compile_error_class<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_compile_error) }
}

/// Predefined class `DivisionByZeroError`.
#[inline]
pub fn division_by_zero_error<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_division_by_zero_error) }
}

/// PHP Throwable, can cause throwing an exception when setting to
/// [crate::values::ZVal].
pub trait Throwable: error::Error {
    fn get_class(&self) -> &ClassEntry;

    #[inline]
    fn get_code(&self) -> Option<i64> {
        Some(0)
    }

    #[inline]
    fn get_message(&self) -> Option<String> {
        Some(self.to_string())
    }

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

/// Type of [std::result::Result]<T, [crate::Error]>.
pub type Result<T> = result::Result<T, self::Error>;

/// Crate level Error, which also can become an exception in php.
///
/// As a php exception, will throw `ErrorException` when the item not implement
/// [ToThrowable].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromBytesWithNul(#[from] FromBytesWithNulError),

    #[error(transparent)]
    Boxed(#[from] Box<dyn error::Error>),

    #[error(transparent)]
    Throw(#[from] ThrowObject),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    ClassNotFound(#[from] ClassNotFoundError),

    #[error(transparent)]
    ArgumentCount(#[from] ArgumentCountError),

    #[error(transparent)]
    StateType(#[from] StateTypeError),

    #[error(transparent)]
    CallFunction(#[from] CallFunctionError),

    #[error(transparent)]
    CallMethod(#[from] CallMethodError),

    #[error(transparent)]
    InitializeObject(#[from] InitializeObjectError),

    #[error(transparent)]
    NotRefCountedType(#[from] NotRefCountedTypeError),

    #[error(transparent)]
    ExpectType(#[from] ExpectTypeError),

    #[error(transparent)]
    NotImplementThrowable(#[from] NotImplementThrowableError),
}

impl Error {
    pub fn boxed(e: impl error::Error + 'static) -> Self {
        Self::Boxed(Box::new(e))
    }

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
            Error::Type(e) => Throwable::get_class(e),
            Error::ClassNotFound(e) => Throwable::get_class(e),
            Error::ArgumentCount(e) => Throwable::get_class(e),
            Error::StateType(e) => Throwable::get_class(e),
            Error::CallFunction(e) => Throwable::get_class(e),
            Error::CallMethod(e) => Throwable::get_class(e),
            Error::InitializeObject(e) => Throwable::get_class(e),
            Error::NotRefCountedType(e) => Throwable::get_class(e),
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
            Error::Type(e) => Throwable::get_code(e),
            Error::ClassNotFound(e) => Throwable::get_code(e),
            Error::ArgumentCount(e) => Throwable::get_code(e),
            Error::StateType(e) => Throwable::get_code(e),
            Error::CallFunction(e) => Throwable::get_code(e),
            Error::CallMethod(e) => Throwable::get_code(e),
            Error::InitializeObject(e) => Throwable::get_code(e),
            Error::NotRefCountedType(e) => Throwable::get_code(e),
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
            Error::Type(e) => Throwable::get_message(e),
            Error::ClassNotFound(e) => Throwable::get_message(e),
            Error::ArgumentCount(e) => Throwable::get_message(e),
            Error::StateType(e) => Throwable::get_message(e),
            Error::CallFunction(e) => Throwable::get_message(e),
            Error::CallMethod(e) => Throwable::get_message(e),
            Error::InitializeObject(e) => Throwable::get_message(e),
            Error::NotRefCountedType(e) => Throwable::get_message(e),
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
            Error::Type(e) => Throwable::to_object(e),
            Error::ClassNotFound(e) => Throwable::to_object(e),
            Error::ArgumentCount(e) => Throwable::to_object(e),
            Error::StateType(e) => Throwable::to_object(e),
            Error::CallFunction(e) => Throwable::to_object(e),
            Error::CallMethod(e) => Throwable::to_object(e),
            Error::InitializeObject(e) => Throwable::to_object(e),
            Error::NotRefCountedType(e) => Throwable::to_object(e),
            Error::ExpectType(e) => Throwable::to_object(e),
            Error::NotImplementThrowable(e) => Throwable::to_object(e),
        }
    }
}

#[derive(Debug)]
pub struct ThrowObject(ZObject);

impl ThrowObject {
    pub fn new(obj: ZObject) -> result::Result<Self, NotImplementThrowableError> {
        if !obj.get_class().instance_of(throwable_class()) {
            return Err(NotImplementThrowableError);
        }
        Ok(Self(obj))
    }

    #[inline]
    pub fn from_throwable(mut t: impl Throwable) -> Self {
        Self::from_result(t.to_object())
    }

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

#[derive(Debug, thiserror::Error, Constructor)]
#[error("type error: {message}")]
pub struct TypeError {
    message: String,
}

impl Throwable for TypeError {
    #[inline]
    fn get_class(&self) -> &ClassEntry {
        type_error_class()
    }
}

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

#[derive(Debug, thiserror::Error)]
#[error(
    "Actual State type in generic type parameter isn't the state type registered in the class, \
     please confirm the real state type, or use StatelessClassEntry"
)]
pub struct StateTypeError;

impl Throwable for StateTypeError {
    fn get_class(&self) -> &ClassEntry {
        error_class()
    }
}

#[derive(Debug, thiserror::Error, Constructor)]
#[error("{function_name}(): expects at least {expect_count} parameter(s), {given_count} given")]
pub struct ArgumentCountError {
    function_name: String,
    expect_count: usize,
    given_count: usize,
}

impl Throwable for ArgumentCountError {
    fn get_class(&self) -> &ClassEntry {
        #[cfg(phper_version_id_gte_70100)]
        {
            argument_count_error_class()
        }

        #[cfg(not(phper_version_id_gte_70100))]
        {
            type_error_class()
        }
    }
}

#[derive(Debug, thiserror::Error, Constructor)]
#[error("Invalid call to {fn_name}")]
pub struct CallFunctionError {
    fn_name: String,
}

impl Throwable for CallFunctionError {
    fn get_class(&self) -> &ClassEntry {
        error_class()
    }
}

#[derive(Debug, thiserror::Error, Constructor)]
#[error("Invalid call to {class_name}::{method_name}")]
pub struct CallMethodError {
    class_name: String,
    method_name: String,
}

impl Throwable for CallMethodError {
    fn get_class(&self) -> &ClassEntry {
        error_class()
    }
}

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

#[derive(Debug, thiserror::Error)]
#[error("the type is not refcounted")]
pub struct NotRefCountedTypeError;

impl Throwable for NotRefCountedTypeError {
    fn get_class(&self) -> &ClassEntry {
        type_error_class()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Cannot throw objects that do not implement Throwable")]
pub struct NotImplementThrowableError;

impl Throwable for NotImplementThrowableError {
    fn get_class(&self) -> &ClassEntry {
        type_error_class()
    }
}

pub struct ExceptionGuard(PhantomData<()>);

impl ExceptionGuard {
    pub fn new() -> Self {
        unsafe {
            zend_exception_save();
        }
        Self(PhantomData)
    }
}

impl Drop for ExceptionGuard {
    fn drop(&mut self) {
        unsafe {
            zend_exception_restore();
        }
    }
}
