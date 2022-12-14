// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to PHP types.

use crate::{c_str, sys::*};
use derive_more::From;
use std::{ffi::CStr, fmt::Display, os::raw::c_int};

/// Wrapper of PHP type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub const REFERENCE: TypeInfo = TypeInfo::from_raw(IS_REFERENCE);
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

    pub const fn is_reference(self) -> bool {
        get_base_type_by_raw(self.t) == IS_REFERENCE
    }

    pub const fn get_base_type(self) -> TypeInfo {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    /// Can be sure, `zend_get_type_by_const` returns the const string, So this
    /// method returns `&'static CStr`.
    #[inline]
    pub fn get_base_type_name(self) -> &'static CStr {
        unsafe {
            let t = get_base_type_by_raw(self.t);

            if t == IS_UNDEF {
                return c_str!("undef");
            }
            if t == IS_REFERENCE {
                return c_str!("reference");
            }

            let s = zend_get_type_by_const(t as c_int);
            let s = CStr::from_ptr(s);

            // Compact with PHP7.
            let bs = s.to_bytes();
            if bs == b"boolean" {
                return c_str!("bool");
            }
            if bs == b"integer" {
                return c_str!("int");
            }

            s
        }
    }
}

impl From<u32> for TypeInfo {
    fn from(n: u32) -> Self {
        Self::from_raw(n)
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.get_base_type_name().to_str().unwrap_or("unknown");
        Display::fmt(t, f)
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
