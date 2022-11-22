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

use crate::{strings::ZStr, sys::*, values::ZVal};
use derive_more::From;
use std::{ffi::CStr, fmt::Display, os::raw::c_int};

type ZendType = zend_type;

/// Wrapper of PHP type.
#[derive(Clone, Copy, Debug)]
pub struct TypeInfo {
    inner: ZendType,
}

impl TypeInfo {
    pub const ARRAY: TypeInfo = TypeInfo::from_type_mask(IS_ARRAY);
    pub const BOOL: TypeInfo = TypeInfo::from_type_mask(_IS_BOOL);
    pub const CALLABLE: TypeInfo = TypeInfo::from_type_mask(IS_CALLABLE);
    pub const DOUBLE: TypeInfo = TypeInfo::from_type_mask(IS_DOUBLE);
    pub const FALSE: TypeInfo = TypeInfo::from_type_mask(IS_FALSE);
    #[cfg(any(
        all(phper_major_version = "7", any(phper_minor_version = "1", phper_minor_version = "2", phper_minor_version = "3", phper_minor_version = "4"))
        phper_major_version = "8"
    ))]
    pub const ITERABLE: TypeInfo = TypeInfo::from_type_mask(IS_ITERABLE);
    pub const LONG: TypeInfo = TypeInfo::from_type_mask(IS_LONG);
    #[cfg(phper_major_version = "8")]
    pub const MIXED: TypeInfo = TypeInfo::from_type_mask(IS_MIXED);
    #[cfg(all(
        phper_major_version = "8",
        any(phper_minor_version = "1", phper_minor_version = "2")
    ))]
    pub const NEVER: TypeInfo = TypeInfo::from_type_mask(IS_NEVER);
    pub const NULL: TypeInfo = TypeInfo::from_type_mask(IS_NULL);
    pub const OBJECT: TypeInfo = TypeInfo::from_type_mask(IS_OBJECT);
    pub const RESOURCE: TypeInfo = TypeInfo::from_type_mask(IS_RESOURCE);
    pub const STRING: TypeInfo = TypeInfo::from_type_mask(IS_STRING);
    pub const TRUE: TypeInfo = TypeInfo::from_type_mask(IS_TRUE);
    pub const UNDEF: TypeInfo = TypeInfo::from_type_mask(IS_UNDEF);
    pub const VOID: TypeInfo = TypeInfo::from_type_mask(IS_VOID);
}

impl TypeInfo {
    pub const fn from_type_mask(mask: u32) -> Self {
        Self {
            inner: ZendType {
                ptr: std::ptr::null::<std::ffi::c_void>() as *mut std::ffi::c_void,
                type_mask: mask,
            },
        }
    }

    pub fn from_zval(zval: &ZVal) -> Self {
        let mask = unsafe { phper_z_type_info_p(zval.as_ptr()) };
        if (mask & IS_OBJECT) != 0 {
            let name_ptr = unsafe {
                let obj = phper_z_obj_p(zval.as_ptr());
                let ce = (*obj).ce;
                phper_zend_object_release(obj);
                (*ce).name as *mut std::ffi::c_void
            };

            Self {
                inner: ZendType {
                    ptr: name_ptr,
                    type_mask: mask | _ZEND_TYPE_NAME_BIT,
                },
            }
        } else {
            Self::from_type_mask(mask)
        }
    }

    pub const fn from_class_entry(ce: &zend_class_entry) -> Self {
        let name_ptr = ce.name as *mut std::ffi::c_void;

        Self {
            inner: ZendType {
                ptr: name_ptr,
                type_mask: _ZEND_TYPE_NAME_BIT,
            },
        }
    }

    pub const fn from_raw(inner: ZendType) -> Self {
        Self { inner }
    }

    pub const fn into_raw(self) -> ZendType {
        self.inner
    }

    pub const fn undef() -> TypeInfo {
        Self::from_type_mask(IS_UNDEF)
    }

    pub const fn null() -> TypeInfo {
        Self::from_type_mask(IS_NULL)
    }

    pub const fn bool(b: bool) -> TypeInfo {
        Self::from_type_mask(if b { IS_TRUE } else { IS_FALSE })
    }

    pub const fn long() -> TypeInfo {
        Self::from_type_mask(IS_LONG)
    }

    pub const fn double() -> TypeInfo {
        Self::from_type_mask(IS_DOUBLE)
    }

    pub const fn string() -> TypeInfo {
        Self::from_type_mask(IS_STRING)
    }

    pub const fn array() -> TypeInfo {
        Self::from_type_mask(IS_ARRAY)
    }

    pub const fn array_ex() -> TypeInfo {
        Self::from_type_mask(IS_ARRAY_EX)
    }

    pub const fn object() -> TypeInfo {
        Self::from_type_mask(IS_OBJECT)
    }

    pub const fn object_ex() -> TypeInfo {
        Self::from_type_mask(IS_OBJECT_EX)
    }

    #[cfg(any(
        all(phper_major_version = "7", phper_minor_version = "4")
        phper_major_version = "8"
    ))]
    pub const fn is_complex(self) -> bool {
        (self.inner.type_mask & _ZEND_TYPE_KIND_MASK) != 0
    }

    pub const fn has_name(self) -> bool {
        (self.inner.type_mask & _ZEND_TYPE_NAME_BIT) != 0
    }

    #[cfg(any(
        all(phper_major_version = "7", phper_minor_version = "4")
        phper_major_version = "8"
    ))]
    pub const fn has_list(self) -> bool {
        (self.inner.type_mask & _ZEND_TYPE_LIST_BIT) != 0
    }

    #[cfg(all(
        phper_major_version = "8",
        any(phper_minor_version = "1", phper_minor_version = "2")
    ))]
    pub const fn is_union(self) -> bool {
        (self.inner.type_mask & _ZEND_TYPE_UNION_BIT) != 0
    }

    #[cfg(all(
        phper_major_version = "8",
        any(phper_minor_version = "1", phper_minor_version = "2")
    ))]
    pub const fn is_intersaction(self) -> bool {
        (self.inner.type_mask & _ZEND_TYPE_INTERSECTION_BIT) != 0
    }

    pub const fn is_undef(self) -> bool {
        self.inner.type_mask == IS_UNDEF
    }

    pub const fn is_null(self) -> bool {
        self.inner.type_mask == IS_NULL
    }

    pub const fn is_bool(self) -> bool {
        self.is_true() || self.is_false()
    }

    pub const fn is_true(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_TRUE
    }

    #[cfg(phper_major_version = "8")]
    pub const fn is_mixed(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_MIXED
    }

    pub const fn is_void(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_VOID
    }

    #[cfg(all(
        phper_major_version = "8",
        any(phper_minor_version = "1", phper_minor_version = "2")
    ))]
    pub const fn is_never(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_NEVER
    }

    pub const fn is_callable(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_CALLABLE
    }

    #[cfg(any(
        all(phper_major_version = "7", any(phper_minor_version = "1", phper_minor_version = "2", phper_minor_version = "3", phper_minor_version = "4"))
        phper_major_version = "8"
    ))]
    pub const fn is_iterable(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_ITERABLE
    }

    pub const fn is_false(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_FALSE
    }

    pub const fn is_long(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_LONG
    }

    pub const fn is_double(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_DOUBLE
    }

    pub const fn is_string(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_STRING
    }

    pub const fn is_array(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_ARRAY
    }

    pub const fn is_object(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_OBJECT
    }

    pub const fn is_resource(self) -> bool {
        get_base_type_by_raw(self.inner.type_mask) == IS_RESOURCE
    }

    pub const fn is_indirect(self) -> bool {
        self.inner.type_mask == IS_INDIRECT
    }

    pub const fn get_base_type(self) -> TypeInfo {
        Self::from_type_mask(get_base_type_by_raw(self.inner.type_mask))
    }

    pub fn get_base_type_name(self) -> crate::Result<String> {
        get_type_by_const(self.inner.type_mask)
    }

    pub fn name(self) -> crate::Result<String> {
        if !self.has_name() {
            self.get_base_type_name()
        } else {
            // SAFETY: `self.inner.ptr` is not null (or at least shouldn't be...)
            let z = unsafe { ZStr::from_ptr(self.inner.ptr as *const _zend_string) };

            Ok(z.to_str()?.to_string())
        }
    }
}

impl From<u32> for TypeInfo {
    fn from(n: u32) -> Self {
        Self::from_type_mask(n)
    }
}

impl Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = if let Ok(name) = self.name() {
            name
        } else {
            "unknown".to_string()
        };

        Display::fmt(&t, f)
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
