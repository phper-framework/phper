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

use crate::sys::*;
use derive_more::From;
use std::{
    ffi::CStr,
    fmt::{self, Debug, Display},
    os::raw::c_int,
};

/// Wrapper of PHP type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TypeInfo {
    t: u32,
}

impl TypeInfo {
    /// Array type info.
    pub const ARRAY: TypeInfo = TypeInfo::from_raw(IS_ARRAY);
    /// Boolean type info.
    pub const BOOL: TypeInfo = TypeInfo::from_raw(_IS_BOOL);
    /// Double number type info.
    pub const DOUBLE: TypeInfo = TypeInfo::from_raw(IS_DOUBLE);
    /// Long number type info.
    pub const LONG: TypeInfo = TypeInfo::from_raw(IS_LONG);
    /// Null type info.
    pub const NULL: TypeInfo = TypeInfo::from_raw(IS_NULL);
    /// Object type info.
    pub const OBJECT: TypeInfo = TypeInfo::from_raw(IS_OBJECT);
    /// Reference type info.
    pub const REFERENCE: TypeInfo = TypeInfo::from_raw(IS_REFERENCE);
    /// Resource type info.
    pub const RESOURCE: TypeInfo = TypeInfo::from_raw(IS_RESOURCE);
    /// String type info.
    pub const STRING: TypeInfo = TypeInfo::from_raw(IS_STRING);
    /// Undefined type info.
    pub const UNDEF: TypeInfo = TypeInfo::from_raw(IS_UNDEF);
}

impl TypeInfo {
    /// Construct [`TypeInfo`] from raw number.
    pub const fn from_raw(t: u32) -> Self {
        Self { t }
    }

    /// Transfers [`TypeInfo`] to raw number.
    pub const fn into_raw(self) -> u32 {
        self.t
    }

    /// Detects if the [`TypeInfo`] is undefined.
    pub const fn is_undef(self) -> bool {
        self.t == IS_UNDEF
    }

    /// Detects if the [`TypeInfo`] is null.
    pub const fn is_null(self) -> bool {
        self.t == IS_NULL
    }

    /// Detects if the [`TypeInfo`] is boolean.
    pub const fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    /// Detects if the [`TypeInfo`] is true.
    pub const fn is_true(self) -> bool {
        get_base_type_by_raw(self.t) == IS_TRUE
    }

    /// Detects if the [`TypeInfo`] is false.
    pub const fn is_false(self) -> bool {
        get_base_type_by_raw(self.t) == IS_FALSE
    }

    /// Detects if the [`TypeInfo`] is long.
    pub const fn is_long(self) -> bool {
        get_base_type_by_raw(self.t) == IS_LONG
    }

    /// Detects if the [`TypeInfo`] is double.
    pub const fn is_double(self) -> bool {
        get_base_type_by_raw(self.t) == IS_DOUBLE
    }

    /// Detects if the [`TypeInfo`] is string.
    pub const fn is_string(self) -> bool {
        get_base_type_by_raw(self.t) == IS_STRING
    }

    /// Detects if the [`TypeInfo`] is array.
    pub const fn is_array(self) -> bool {
        get_base_type_by_raw(self.t) == IS_ARRAY
    }

    /// Detects if the [`TypeInfo`] is object.
    pub const fn is_object(self) -> bool {
        get_base_type_by_raw(self.t) == IS_OBJECT
    }

    /// Detects if the [`TypeInfo`] is resource.
    pub const fn is_resource(self) -> bool {
        get_base_type_by_raw(self.t) == IS_RESOURCE
    }

    /// Detects if the [`TypeInfo`] is reference.
    pub const fn is_reference(self) -> bool {
        get_base_type_by_raw(self.t) == IS_REFERENCE
    }

    /// Transfers [`TypeInfo`] to the base type info.
    pub const fn get_base_type(self) -> TypeInfo {
        Self::from_raw(get_base_type_by_raw(self.t))
    }

    /// Gets the name of base type.
    ///
    /// Can be sure, `zend_get_type_by_const` returns the const string, So this
    /// method returns `&'static CStr`.
    #[inline]
    pub fn get_base_type_name(self) -> &'static CStr {
        unsafe {
            let t = get_base_type_by_raw(self.t);

            if t == IS_UNDEF {
                return c"undef";
            }
            if t == IS_REFERENCE {
                return c"reference";
            }

            let s = zend_get_type_by_const(t as c_int);
            let s = CStr::from_ptr(s);

            // Compact with PHP7.
            let bs = s.to_bytes();
            if bs == b"boolean" {
                return c"bool";
            }
            if bs == b"integer" {
                return c"int";
            }

            s
        }
    }
}

impl Debug for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypeInfo")
            .field("base_name", &self.get_base_type_name())
            .field("base", &self.get_base_type().t)
            .field("raw", &self.t)
            .finish()
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

/// Copyable value, used in constant and class property.
#[derive(From)]
pub enum Scalar {
    /// Null.
    Null,
    /// Boolean.
    Bool(bool),
    /// Long.
    I64(i64),
    /// Double.
    F64(f64),
    /// String
    String(String),
    /// Binary string.
    Bytes(Vec<u8>),
}

impl From<()> for Scalar {
    fn from(_: ()) -> Self {
        Self::Null
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
