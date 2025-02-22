// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_ini_entry_def].

use crate::sys::*;
use std::{
    ffi::{CStr, c_int},
    mem::zeroed,
    os::raw::c_char,
    ptr::null_mut,
    str,
};

/// Get the global registered configuration value.
///
/// # Examples
///
/// ```no_run
/// use phper::ini::ini_get;
/// use std::ffi::CStr;
///
/// let _foo = ini_get::<bool>("FOO");
/// let _bar = ini_get::<Option<&CStr>>("BAR");
/// ```
pub fn ini_get<T: FromIniValue>(name: &str) -> T {
    T::from_ini_value(name)
}

/// Configuration changeable policy.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Policy {
    /// Entry can be set anywhere.
    All = PHP_INI_ALL,
    /// Entry can be set in user scripts (like with `ini_set()`) or in the
    /// Windows registry. Entry can be set in `.user.ini`.
    User = PHP_INI_USER,
    /// Entry can be set in `php.ini`, `.htaccess`, `httpd.conf` or `.user.ini`.
    Perdir = PHP_INI_PERDIR,
    /// Entry can be set in `php.ini` or `httpd.conf`.
    System = PHP_INI_SYSTEM,
}

/// The Type which can transform to an ini value.
pub trait IntoIniValue {
    /// transform to an ini value.
    fn into_ini_value(self) -> String;
}

impl IntoIniValue for bool {
    #[inline]
    fn into_ini_value(self) -> String {
        if self { "1".to_owned() } else { "0".to_owned() }
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

/// The Type which can transform from an ini key name.
///
/// For php7, the zend_ini_* functions receive ini name as `*mut c_char`, but I
/// think it's immutable.
pub trait FromIniValue {
    /// transform from an ini key name.
    fn from_ini_value(name: &str) -> Self;
}

impl FromIniValue for bool {
    #[allow(clippy::useless_conversion)]
    fn from_ini_value(name: &str) -> Self {
        let s = <Option<&CStr>>::from_ini_value(name);
        [Some(c"1"), Some(c"true"), Some(c"on"), Some(c"On")].contains(&s)
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
    pub(crate) fn entry(&self) -> zend_ini_entry_def {
        create_ini_entry_ex(&self.name, &self.default_value, self.policy as u32)
    }
}

fn create_ini_entry_ex(name: &str, default_value: &str, modifiable: u32) -> zend_ini_entry_def {
    #[cfg(any(
        phper_major_version = "8",
        all(
            phper_major_version = "7",
            any(phper_minor_version = "4", phper_minor_version = "3")
        )
    ))]
    let (modifiable, name_length) = (modifiable as std::os::raw::c_uchar, name.len() as u16);

    #[cfg(all(
        phper_major_version = "7",
        any(
            phper_minor_version = "2",
            phper_minor_version = "1",
            phper_minor_version = "0",
        )
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

unsafe fn entries(ini_entries: &[IniEntity]) -> *const zend_ini_entry_def {
    unsafe {
        let mut entries = Vec::with_capacity(ini_entries.len() + 1);

        ini_entries.iter().for_each(|entity| {
            // Ini entity will exist throughout the whole application life cycle.
            entries.push(entity.entry());
        });

        entries.push(zeroed::<zend_ini_entry_def>());

        Box::into_raw(entries.into_boxed_slice()).cast()
    }
}

pub(crate) fn register(ini_entries: &[IniEntity], module_number: c_int) {
    unsafe {
        zend_register_ini_entries(entries(ini_entries), module_number);
    }
}

pub(crate) fn unregister(module_number: c_int) {
    unsafe {
        zend_unregister_ini_entries(module_number);
    }
}
