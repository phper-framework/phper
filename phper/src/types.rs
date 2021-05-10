//! Apis relate to PHP types.

use crate::sys::*;
use num_traits::cast::FromPrimitive;
use std::{ffi::CStr, os::raw::c_int};

#[derive(FromPrimitive, PartialEq, Clone, Copy)]
#[repr(u32)]
#[non_exhaustive]
pub enum Type {
    Undef = IS_UNDEF,
    Null = IS_NULL,
    False = IS_FALSE,
    True = IS_TRUE,
    Long = IS_LONG,
    Double = IS_DOUBLE,
    String = IS_STRING,
    StringEx = IS_STRING_EX,
    Array = IS_ARRAY,
    ArrayEx = IS_ARRAY_EX,
    Object = IS_OBJECT,
    ObjectEx = IS_OBJECT_EX,
    Resource = IS_RESOURCE,
    Reference = IS_REFERENCE,
    ConstantAst = IS_CONSTANT_AST,
    IsCallable = IS_CALLABLE,
}

impl Type {
    #[inline]
    pub fn is_null(self) -> bool {
        self == Type::Null
    }

    #[inline]
    pub fn is_bool(self) -> bool {
        matches!(self, Type::True | Type::False)
    }

    #[inline]
    pub fn is_true(self) -> bool {
        self == Type::True
    }

    #[inline]
    pub fn is_false(self) -> bool {
        self == Type::False
    }

    #[inline]
    pub fn is_long(self) -> bool {
        self == Type::Long
    }

    #[inline]
    pub fn is_double(self) -> bool {
        self == Type::Double
    }

    #[inline]
    pub fn is_string(self) -> bool {
        matches!(self, Type::String | Type::StringEx)
    }

    #[inline]
    pub fn is_array(self) -> bool {
        matches!(self, Type::Array | Type::ArrayEx)
    }

    #[inline]
    pub fn is_object(self) -> bool {
        matches!(self, Type::Object | Type::ObjectEx)
    }
}

impl From<u32> for Type {
    fn from(n: u32) -> Self {
        match FromPrimitive::from_u32(n) {
            Some(t) => t,
            None => unreachable!("Type is not exhaustive, should contains: {}", n),
        }
    }
}

pub(crate) fn get_type_by_const(t: u32) -> crate::Result<String> {
    unsafe {
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
