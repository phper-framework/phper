use crate::sys::zend_ini_entry_def;
use std::cell::UnsafeCell;

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
