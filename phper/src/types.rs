// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to PHP types.

use crate::sys::*;
use derive_more::From;
use std::{ffi::CStr, fmt::Display, os::raw::c_int};

/// Wrapper of PHP type.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TypeInfo {
    t: u32,
}

impl TypeInfo {
    pub const ARRAY: TypeInfo = TypeInfo::from_raw(IS_ARRAY);
    pub const BOOL: TypeInfo = TypeInfo::from_raw(_IS_BOOL);
    pub const DOUBLE: TypeInfo = TypeInfo::from_raw(IS_DOUBLE);
    pub const LONG: TypeInfo = TypeInfo::from_raw(IS_LONG);
    pub const NULL: TypeInfo = TypeInfo::from_raw(IS_NULL);
    pub const OBJECT: TypeInfo = TypeInfo::from_raw(IS_OBJECT);
    pub const RESOURCE: TypeInfo = TypeInfo::from_raw(IS_RESOURCE);
    pub const STRING: TypeInfo = TypeInfo::from_raw(IS_STRING);
    pub const UNDEF: TypeInfo = TypeInfo::from_raw(IS_UNDEF);
}

impl TypeInfo {
    pub const fn from_raw(t: u32) -> Self {
        Self { t }
    }

    pub const fn into_raw(self) -> u32 {
        self.t
    }

    pub const fn undef() -> TypeInfo {
        Self::from_raw(IS_UNDEF)
    }

    pub const fn null() -> TypeInfo {
        Self::from_raw(IS_NULL)
    }

    pub const fn bool(b: bool) -> TypeInfo {
        Self::from_raw(if b { IS_TRUE } else { IS_FALSE })
    }

    pub const fn long() -> TypeInfo {
        Self::from_raw(IS_LONG)
    }

    pub const fn double() -> TypeInfo {
        Self::from_raw(IS_DOUBLE)
    }

    pub const fn string() -> TypeInfo {
        Self::from_raw(IS_STRING)
    }

    pub const fn array() -> TypeInfo {
        Self::from_raw(IS_ARRAY)
    }

    pub const fn array_ex() -> TypeInfo {
        Self::from_raw(IS_ARRAY_EX)
    }

    pub const fn object() -> TypeInfo {
        Self::from_raw(IS_OBJECT)
    }

    pub const fn object_ex() -> TypeInfo {
        Self::from_raw(IS_OBJECT_EX)
    }

    pub const fn is_undef(self) -> bool {
        self.t == IS_UNDEF
    }

    pub const fn is_null(self) -> bool {
        self.t == IS_NULL
    }

    pub const fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    pub const fn is_true(self) -> bool {
        get_base_type_by_raw(self.t) == IS_TRUE
    }

    pub const fn is_false(self) -> bool {
        get_base_type_by_raw(self.t) == IS_FALSE
    }

    pub const fn is_long(self) -> bool {
        get_base_type_by_raw(self.t) == IS_LONG
    }

    pub const fn is_double(self) -> bool {
        get_base_type_by_raw(self.t) == IS_DOUBLE
    }

    pub const fn is_string(self) -> bool {
        get_base_type_by_raw(self.t) == IS_STRING
    }

    pub const fn is_array(self) -> bool {
        get_base_type_by_raw(self.t) == IS_ARRAY
    }

    pub const fn is_object(self) -> bool {
        get_base_type_by_raw(self.t) == IS_OBJECT
    }

    pub const fn is_resource(self) -> bool {
        get_base_type_by_raw(self.t) == IS_RESOURCE
    }

    pub const fn is_indirect(self) -> bool {
        self.t == IS_INDIRECT
    }

    pub const fn get_base_type(self) -> TypeInfo {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    pub fn get_base_type_name(self) -> crate::Result<String> {
        get_type_by_const(self.t)
    }
}

impl From<u32> for TypeInfo {
    fn from(n: u32) -> Self {
        Self::from_raw(n)
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = if self.is_null() {
            "null"
        } else if self.is_bool() {
            "bool"
        } else if self.is_long() {
            "int"
        } else if self.is_double() {
            "float"
        } else if self.is_string() {
            "string"
        } else if self.is_array() {
            "array"
        } else if self.is_object() {
            "object"
        } else if self.is_resource() {
            "resource"
        } else {
            "unknown"
        };
        Display::fmt(t, f)
    }
}

fn get_type_by_const(mut t: u32) -> crate::Result<String> {
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

const fn get_base_type_by_raw(t: u32) -> u32 {
    t & !(!0 << Z_TYPE_FLAGS_SHIFT)
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
