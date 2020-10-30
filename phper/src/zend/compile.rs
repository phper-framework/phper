use crate::sys::zend_internal_arg_info;
use std::cell::UnsafeCell;

pub struct InternalArgInfos<const N: usize> {
    inner: UnsafeCell<[zend_internal_arg_info; N]>,
}

impl<const N: usize> InternalArgInfos<N> {
    pub const fn new(inner: [zend_internal_arg_info; N]) -> Self {
        Self { inner: UnsafeCell::new(inner) }
    }

    pub const fn get(&self) -> *const zend_internal_arg_info {
        self.inner.get().cast()
    }
}

unsafe impl<const N: usize> Sync for InternalArgInfos<N> {}
