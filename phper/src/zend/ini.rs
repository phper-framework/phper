use crate::sys::{
    zend_ini_entry, zend_ini_entry_def, zend_string, OnUpdateString, PHP_INI_ALL, PHP_INI_PERDIR,
    PHP_INI_SYSTEM, PHP_INI_USER,
};
use std::{
    cell::Cell,
    mem::{size_of, transmute},
    os::raw::{c_int, c_void},
    ptr::null_mut,
    sync::atomic::AtomicPtr,
};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Policy {
    All = PHP_INI_ALL,
    User = PHP_INI_USER,
    Perdir = PHP_INI_PERDIR,
    System = PHP_INI_SYSTEM,
}

pub(crate) struct IniEntity {
    name: String,
    value: usize,
    default_value: String,
    policy: Policy,
}

impl IniEntity {
    pub(crate) fn new(name: impl ToString, default_value: impl ToString, policy: Policy) -> Self {
        Self {
            name: name.to_string(),
            value: 0,
            default_value: default_value.to_string(),
            policy,
        }
    }

    // TODO Pass the logic of multi type item.
    pub(crate) fn ini_entry_def(&self) -> zend_ini_entry_def {
        let arg2: Box<*mut c_void> = Box::new(null_mut());
        create_ini_entry_ex(
            &self.name,
            &self.default_value,
            Some(OnUpdateString),
            self.policy as u32,
            Box::leak(arg2).cast(),
        )
    }
}

pub type Mh = unsafe extern "C" fn(
    *mut zend_ini_entry,
    *mut zend_string,
    *mut c_void,
    *mut c_void,
    *mut c_void,
    c_int,
) -> c_int;

const fn ini_entry_def_end() -> zend_ini_entry_def {
    unsafe { transmute([0u8; size_of::<zend_ini_entry_def>()]) }
}

#[repr(C)]
struct ZendIniEntriesWithEnd<const N: usize>([zend_ini_entry_def; N], zend_ini_entry_def);

pub struct IniEntries<const N: usize> {
    inner: Cell<ZendIniEntriesWithEnd<N>>,
}

impl<const N: usize> IniEntries<N> {
    pub const fn new(inner: [zend_ini_entry_def; N]) -> Self {
        Self {
            inner: Cell::new(ZendIniEntriesWithEnd(inner, ini_entry_def_end())),
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const zend_ini_entry_def {
        self.inner.as_ptr().cast()
    }
}

unsafe impl<const N: usize> Sync for IniEntries<N> {}

pub const fn create_ini_entry(
    name: &str,
    default_value: &str,
    modifiable: u32,
) -> zend_ini_entry_def {
    create_ini_entry_ex(name, default_value, None, modifiable, null_mut())
}

pub const fn create_ini_entry_ex(
    name: &str,
    default_value: &str,
    on_modify: Option<Mh>,
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
        mh_arg1: 0 as *mut _,
        mh_arg2: arg2,
        mh_arg3: null_mut(),
        value: default_value.as_ptr().cast(),
        displayer: None,
        modifiable,
        name_length,
        value_length: default_value.len() as u32,
    }
}
