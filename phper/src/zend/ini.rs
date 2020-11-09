use crate::sys::{zend_ini_entry, zend_ini_entry_def, zend_string};
use std::{
    cell::Cell,
    mem::{size_of, transmute},
    os::raw::{c_int, c_void},
};

pub type Mh = unsafe extern "C" fn(
    *mut zend_ini_entry,
    *mut zend_string,
    *mut c_void,
    *mut c_void,
    *mut c_void,
    c_int,
) -> c_int;

pub const fn ini_entry_def_end() -> zend_ini_entry_def {
    unsafe { transmute([0u8; size_of::<zend_ini_entry_def>()]) }
}

pub struct IniEntryDefs<const N: usize> {
    inner: Cell<[zend_ini_entry_def; N]>,
}

impl<const N: usize> IniEntryDefs<N> {
    pub const fn new(inner: [zend_ini_entry_def; N]) -> Self {
        Self {
            inner: Cell::new(inner),
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const zend_ini_entry_def {
        self.inner.as_ptr().cast()
    }
}

unsafe impl<const N: usize> Sync for IniEntryDefs<N> {}
