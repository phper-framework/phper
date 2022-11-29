// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_ini_entry_def].

use crate::sys::*;
use std::{
    ffi::CStr,
    mem::{zeroed, ManuallyDrop},
    os::raw::c_char,
    ptr::null_mut,
    str,
};

pub fn ini_get<T: FromIniValue>(name: &str) -> T {
    T::from_ini_value(name)
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Policy {
    All = PHP_INI_ALL,
    User = PHP_INI_USER,
    Perdir = PHP_INI_PERDIR,
    System = PHP_INI_SYSTEM,
}

/// The Type which can transform to an ini value.
pub trait IntoIniValue {
    fn into_ini_value(self) -> String;
}

impl IntoIniValue for bool {
    #[inline]
    fn into_ini_value(self) -> String {
        if self {
            "1".to_owned()
        } else {
            "0".to_owned()
        }
    }
}

impl IntoIniValue for i64 {
    #[inline]
    fn into_ini_value(self) -> String {
        self.to_string()
    }
}

impl IntoIniValue for f64 {
    #[inline]
    fn into_ini_value(self) -> String {
        self.to_string()
    }
}

impl IntoIniValue for String {
    #[inline]
    fn into_ini_value(self) -> String {
        self
    }
}

/// For php7, the zend_ini_* functions receive ini name as `*mut c_char`, but I
/// think it's immutable.
pub trait FromIniValue {
    fn from_ini_value(name: &str) -> Self;
}

impl FromIniValue for bool {
    #[allow(clippy::useless_conversion)]
    fn from_ini_value(name: &str) -> Self {
        unsafe {
            let name_ptr = name.as_ptr() as *mut u8 as *mut c_char;
            zend_ini_long(name_ptr, name.len().try_into().unwrap(), 0) != 0
        }
    }
}

impl FromIniValue for i64 {
    #[allow(clippy::useless_conversion)]
    fn from_ini_value(name: &str) -> Self {
        unsafe {
            let name_ptr = name.as_ptr() as *mut u8 as *mut c_char;
            zend_ini_long(name_ptr, name.len().try_into().unwrap(), 0)
        }
    }
}

impl FromIniValue for f64 {
    #[allow(clippy::useless_conversion)]
    fn from_ini_value(name: &str) -> Self {
        unsafe {
            let name_ptr = name.as_ptr() as *mut u8 as *mut c_char;
            zend_ini_double(name_ptr, name.len().try_into().unwrap(), 0)
        }
    }
}

impl FromIniValue for Option<&CStr> {
    #[allow(clippy::useless_conversion)]
    fn from_ini_value(name: &str) -> Self {
        unsafe {
            let name_ptr = name.as_ptr() as *mut u8 as *mut c_char;
            let ptr = zend_ini_string_ex(name_ptr, name.len().try_into().unwrap(), 0, null_mut());
            (!ptr.is_null()).then(|| CStr::from_ptr(ptr))
        }
    }
}

pub(crate) struct IniEntity {
    name: String,
    default_value: String,
    policy: Policy,
}

impl IniEntity {
    pub(crate) fn new<T: IntoIniValue>(
        name: impl Into<String>, default_value: T, policy: Policy,
    ) -> Self {
        Self {
            name: name.into(),
            default_value: default_value.into_ini_value(),
            policy,
        }
    }

    #[inline]
    pub(crate) fn entry(&mut self) -> zend_ini_entry_def {
        create_ini_entry_ex(&self.name, &self.default_value, self.policy as u32)
    }
}

fn create_ini_entry_ex(name: &str, default_value: &str, modifiable: u32) -> zend_ini_entry_def {
    #[cfg(any(
        phper_php_version = "8.1",
        phper_php_version = "8.0",
        phper_php_version = "7.4",
        phper_php_version = "7.3",
    ))]
    let (modifiable, name_length) = (modifiable as std::os::raw::c_uchar, name.len() as u16);
    #[cfg(any(
        phper_php_version = "7.2",
        phper_php_version = "7.1",
        phper_php_version = "7.0",
    ))]
    let (modifiable, name_length) = (modifiable as std::os::raw::c_int, name.len() as u32);

    zend_ini_entry_def {
        name: name.as_ptr().cast(),
        on_modify: None,
        mh_arg1: null_mut(),
        mh_arg2: null_mut(),
        mh_arg3: null_mut(),
        value: default_value.as_ptr().cast(),
        displayer: None,
        modifiable,
        name_length,
        value_length: default_value.len() as u32,
    }
}

pub(crate) unsafe fn entries(ini_entries: Vec<IniEntity>) -> *const zend_ini_entry_def {
    let mut entries = Vec::with_capacity(ini_entries.len() + 1);

    ini_entries.into_iter().for_each(|entity| {
        // Ini entity will exist throughout the whole application life cycle.
        let mut entity = ManuallyDrop::new(entity);
        entries.push(entity.entry());
    });

    entries.push(zeroed::<zend_ini_entry_def>());

    Box::into_raw(entries.into_boxed_slice()).cast()
}
