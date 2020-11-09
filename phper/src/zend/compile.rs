use crate::sys::zend_internal_arg_info;
use std::cell::UnsafeCell;

pub const fn internal_arg_info_begin(
    required_num_args: usize,
    return_reference: bool,
) -> zend_internal_arg_info {
    zend_internal_arg_info {
        name: required_num_args as *const _,
        type_: 0,
        pass_by_reference: return_reference as _,
        is_variadic: 0,
    }
}

pub struct MultiInternalArgInfo<const N: usize> {
    inner: UnsafeCell<[zend_internal_arg_info; N]>,
}

impl<const N: usize> MultiInternalArgInfo<N> {
    pub const fn new(inner: [zend_internal_arg_info; N]) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub const fn get(&self) -> *const zend_internal_arg_info {
        self.inner.get().cast()
    }
}

unsafe impl<const N: usize> Sync for MultiInternalArgInfo<N> {}
