use crate::sys::zend_function_entry;
use std::cell::UnsafeCell;

pub struct FunctionEntries<const N: usize> {
    inner: UnsafeCell<[zend_function_entry; N]>,
}

impl<const N: usize> FunctionEntries<N> {
    pub const fn new(inner: [zend_function_entry; N]) -> Self {
        Self { inner: UnsafeCell::new(inner) }
    }

    pub const fn get(&self) -> *const zend_function_entry {
        self.inner.get().cast()
    }
}

unsafe impl<const N: usize> Sync for FunctionEntries<N> {}
