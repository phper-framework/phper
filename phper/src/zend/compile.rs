use crate::sys::{zend_internal_arg_info, zend_uchar};
use std::cell::Cell;

#[repr(C)]
struct ZendInternalArgInfosWithEnd<const N: usize>(
    zend_internal_arg_info,
    [zend_internal_arg_info; N],
);

pub struct MultiInternalArgInfo<const N: usize> {
    inner: Cell<ZendInternalArgInfosWithEnd<N>>,
}

impl<const N: usize> MultiInternalArgInfo<N> {
    pub const fn new(inner: [zend_internal_arg_info; N], return_reference: bool) -> Self {
        Self {
            inner: Cell::new(ZendInternalArgInfosWithEnd(
                zend_internal_arg_info {
                    name: inner.len() as *const _,
                    type_: 0,
                    pass_by_reference: return_reference as zend_uchar,
                    is_variadic: 0,
                },
                inner,
            )),
        }
    }

    pub const fn as_ptr(&self) -> *const zend_internal_arg_info {
        self.inner.as_ptr().cast()
    }
}

unsafe impl<const N: usize> Sync for MultiInternalArgInfo<N> {}
