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
