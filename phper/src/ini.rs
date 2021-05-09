//! Apis relate to [crate::sys::zend_ini_entry_def].

use crate::sys::{
    phper_zend_ini_mh, zend_ini_entry_def, OnUpdateBool, OnUpdateLong, OnUpdateReal,
    OnUpdateString, PHP_INI_ALL, PHP_INI_PERDIR, PHP_INI_SYSTEM, PHP_INI_USER,
};
use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
    ptr::null_mut,
    str,
};

type OnModify = phper_zend_ini_mh;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Policy {
    All = PHP_INI_ALL,
    User = PHP_INI_USER,
    Perdir = PHP_INI_PERDIR,
    System = PHP_INI_SYSTEM,
}

pub(crate) struct StrPtrBox {
    inner: Box<*mut c_char>,
}

impl StrPtrBox {
    pub(crate) unsafe fn to_string(&self) -> Result<String, str::Utf8Error> {
        Ok(CStr::from_ptr(*self.inner).to_str()?.to_string())
    }
}

impl Default for StrPtrBox {
    fn default() -> Self {
        Self {
            inner: Box::new(null_mut()),
        }
    }
}

pub trait IniValue: Default {
    fn on_modify() -> OnModify;

    fn arg2(&mut self) -> *mut c_void {
        &mut *self as *mut _ as *mut c_void
    }
}

impl IniValue for bool {
    fn on_modify() -> OnModify {
        Some(OnUpdateBool)
    }
}

impl IniValue for i64 {
    fn on_modify() -> OnModify {
        Some(OnUpdateLong)
    }
}

impl IniValue for f64 {
    fn on_modify() -> OnModify {
        Some(OnUpdateReal)
    }
}

impl IniValue for StrPtrBox {
    fn on_modify() -> OnModify {
        Some(OnUpdateString)
    }

    fn arg2(&mut self) -> *mut c_void {
        Box::as_mut(&mut self.inner) as *mut _ as *mut c_void
    }
}

pub(crate) struct IniEntity<T: IniValue> {
    name: String,
    value: T,
    default_value: String,
    policy: Policy,
}

impl<T: IniValue> IniEntity<T> {
    pub(crate) fn new(name: impl ToString, default_value: impl ToString, policy: Policy) -> Self {
        Self {
            name: name.to_string(),
            value: Default::default(),
            default_value: default_value.to_string(),
            policy,
        }
    }

    pub(crate) fn value(&self) -> &T {
        &self.value
    }

    pub(crate) unsafe fn ini_entry_def(&mut self) -> zend_ini_entry_def {
        create_ini_entry_ex(
            &self.name,
            &self.default_value,
            <T>::on_modify(),
            self.policy as u32,
            self.value.arg2(),
        )
    }
}

pub(crate) fn create_ini_entry_ex(
    name: &str,
    default_value: &str,
    on_modify: OnModify,
    modifiable: u32,
    arg2: *mut c_void,
) -> zend_ini_entry_def {
    #[cfg(any(
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
        on_modify,
        mh_arg1: null_mut(),
        mh_arg2: arg2,
        mh_arg3: null_mut(),
        value: default_value.as_ptr().cast(),
        displayer: None,
        modifiable,
        name_length,
        value_length: default_value.len() as u32,
    }
}
