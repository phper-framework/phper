use crate::sys::zend_ini_entry_def;
use std::cell::UnsafeCell;
use std::os::raw::{c_int, c_void};
use crate::sys::{zend_ini_entry, zend_string};
use std::mem::{transmute, size_of};

pub type Mh = unsafe extern "C" fn(*mut zend_ini_entry, *mut zend_string, *mut c_void, *mut c_void, *mut c_void, c_int) -> c_int;

pub const fn ini_entry_def_end() -> zend_ini_entry_def {
    unsafe { transmute([0u8; size_of::<zend_ini_entry_def>()]) }
}

pub struct IniEntryDefs<const N: usize> {
    inner: UnsafeCell<[zend_ini_entry_def; N]>,
}

impl<const N: usize> IniEntryDefs<N> {
    pub const fn new(inner: [zend_ini_entry_def; N]) -> Self {
        Self { inner: UnsafeCell::new(inner) }
    }

    #[inline]
    pub const fn get(&self) -> *const zend_ini_entry_def {
        self.inner.get().cast()
    }
}

unsafe impl<const N: usize> Sync for IniEntryDefs<N> {}

struct Entry {
    a: &'static str,
    b: &'static str,
}

struct Entry2 {
    a: &'static str,
    b: &'static str,
}

const fn entry(e: Entry) -> Entry2 {
    let a = e.a;
    let b = e.b;
    Entry2 { a, b }
}
