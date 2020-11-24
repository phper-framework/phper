use crate::sys::{
    zend_internal_arg_info, zend_uchar, ZEND_ACC_PRIVATE, ZEND_ACC_PROTECTED, ZEND_ACC_PUBLIC,
};
use std::{cell::Cell, os::raw::c_char};

#[repr(C)]
struct ZendInternalArgInfosWithEnd<const N: usize>(
    zend_internal_arg_info,
    [zend_internal_arg_info; N],
);

pub struct MultiInternalArgInfo<const N: usize> {
    inner: Cell<ZendInternalArgInfosWithEnd<N>>,
}

impl<const N: usize> MultiInternalArgInfo<N> {
    pub const fn new(
        required_num_args: usize,
        return_reference: bool,
        inner: [zend_internal_arg_info; N],
    ) -> Self {
        Self {
            inner: Cell::new(ZendInternalArgInfosWithEnd(
                create_zend_arg_info(required_num_args as *const _, return_reference),
                inner,
            )),
        }
    }

    pub const fn as_ptr(&self) -> *const zend_internal_arg_info {
        self.inner.as_ptr().cast()
    }

    pub const fn len(&self) -> usize {
        N
    }
}

unsafe impl<const N: usize> Sync for MultiInternalArgInfo<N> {}

pub const fn create_zend_arg_info(
    name: *const c_char,
    pass_by_ref: bool,
) -> zend_internal_arg_info {
    #[cfg(any(
        phper_php_version = "7.4",
        phper_php_version = "7.3",
        phper_php_version = "7.2"
    ))]
    {
        zend_internal_arg_info {
            name,
            type_: 0,
            pass_by_reference: pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }

    #[cfg(any(phper_php_version = "7.1", phper_php_version = "7.0",))]
    {
        zend_internal_arg_info {
            name,
            class_name: std::ptr::null(),
            type_hint: 0,
            allow_null: 0,
            pass_by_reference: pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Visibility {
    Public = ZEND_ACC_PUBLIC,
    Protected = ZEND_ACC_PROTECTED,
    Private = ZEND_ACC_PRIVATE,
}
