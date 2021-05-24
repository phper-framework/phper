//! Apis relate to PHP types.

use crate::sys::*;
use derive_more::From;
use std::{ffi::CStr, os::raw::c_int};

/// Wrapper of PHP type.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Type {
    t: u32,
}

impl Type {
    pub const fn from_raw(t: u32) -> Self {
        Self { t }
    }

    pub const fn into_raw(self) -> u32 {
        self.t
    }

    #[inline]
    pub fn null() -> Type {
        Self::from_raw(IS_NULL)
    }

    #[inline]
    pub fn bool(b: bool) -> Type {
        Self::from_raw(if b { IS_TRUE } else { IS_FALSE })
    }

    #[inline]
    pub fn long() -> Type {
        Self::from_raw(IS_LONG)
    }

    #[inline]
    pub fn double() -> Type {
        Self::from_raw(IS_DOUBLE)
    }

    #[inline]
    pub fn array() -> Type {
        Self::from_raw(IS_ARRAY)
    }

    #[inline]
    pub fn array_ex() -> Type {
        Self::from_raw(IS_ARRAY_EX)
    }

    #[inline]
    pub fn object() -> Type {
        Self::from_raw(IS_OBJECT)
    }

    #[inline]
    pub fn object_ex() -> Type {
        Self::from_raw(IS_OBJECT_EX)
    }

    #[inline]
    pub fn is_undef(self) -> bool {
        self.t == IS_UNDEF
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.t == IS_NULL
    }

    #[inline]
    pub fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    #[inline]
    pub fn is_true(self) -> bool {
        get_base_type_by_raw(self.t) == IS_TRUE
    }

    #[inline]
    pub fn is_false(self) -> bool {
        get_base_type_by_raw(self.t) == IS_FALSE
    }

    #[inline]
    pub fn is_long(self) -> bool {
        get_base_type_by_raw(self.t) == IS_LONG
    }

    #[inline]
    pub fn is_double(self) -> bool {
        get_base_type_by_raw(self.t) == IS_DOUBLE
    }

    #[inline]
    pub fn is_string(self) -> bool {
        get_base_type_by_raw(self.t) == IS_STRING
    }

    #[inline]
    pub fn is_array(self) -> bool {
        get_base_type_by_raw(self.t) == IS_ARRAY
    }

    #[inline]
    pub fn is_object(self) -> bool {
        get_base_type_by_raw(self.t) == IS_OBJECT
    }

    #[inline]
    pub fn is_indirect(self) -> bool {
        self.t == IS_INDIRECT
    }

    pub fn get_base_type(self) -> Type {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    pub fn get_base_type_name(self) -> crate::Result<String> {
        get_type_by_const(self.t)
    }
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        Self::from_raw(n)
    }
}

pub(crate) fn get_type_by_const(mut t: u32) -> crate::Result<String> {
    unsafe {
        t = get_base_type_by_raw(t);
        let s = zend_get_type_by_const(t as c_int);
        let mut s = CStr::from_ptr(s).to_str()?.to_string();

        // Compact with PHP7.
        if s == "boolean" {
            s = "bool".to_string();
        } else if s == "integer" {
            s = "int".to_string();
        }

        Ok(s)
    }
}

#[inline]
pub fn get_base_type_by_raw(mut t: u32) -> u32 {
    t &= !(IS_TYPE_REFCOUNTED << Z_TYPE_FLAGS_SHIFT);

    #[cfg(any(
        phper_major_version = "8",
        all(phper_major_version = "7", phper_minor_version = "4")
    ))]
    {
        t &= !(IS_TYPE_COLLECTABLE << Z_TYPE_FLAGS_SHIFT);
    }

    t
}

#[derive(From)]
pub enum Scalar {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl From<i32> for Scalar {
    fn from(i: i32) -> Self {
        Self::I64(i.into())
    }
}

impl From<&str> for Scalar {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<&[u8]> for Scalar {
    fn from(b: &[u8]) -> Self {
        Self::Bytes(b.to_owned())
    }
}
