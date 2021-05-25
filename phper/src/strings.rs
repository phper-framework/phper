//! Apis relate to [crate::sys::zend_string].

use crate::{
    alloc::{EAllocatable, EBox},
    sys::*,
};
use std::{os::raw::c_char, slice::from_raw_parts, str, str::Utf8Error};

/// Wrapper of [crate::sys::zend_string].
#[repr(transparent)]
pub struct ZendString {
    inner: zend_string,
}

impl ZendString {
    pub fn new(s: &str) -> EBox<Self> {
        unsafe {
            let ptr = phper_zend_string_init(s.as_ptr().cast(), s.len(), false.into()).cast();
            EBox::from_raw(ptr)
        }
    }

    pub unsafe fn from_raw(ptr: *mut zend_string) -> EBox<Self> {
        EBox::from_raw(ptr as *mut ZendString)
    }

    pub(crate) fn from_ptr<'a>(ptr: *mut zend_string) -> &'a Self {
        unsafe {
            let ptr = ptr as *mut Self;
            ptr.as_ref().unwrap()
        }
    }

    pub fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }

    pub fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe {
            let buf = from_raw_parts(
                &self.inner.val as *const c_char as *const u8,
                self.inner.len,
            );
            let string = str::from_utf8(buf)?.to_string();
            Ok(string)
        }
    }
}

impl EAllocatable for ZendString {
    fn free(ptr: *mut Self) {
        unsafe {
            if (*ptr).inner.gc.refcount == 0 {
                phper_zend_string_release(ptr.cast());
            } else {
                (*ptr).inner.gc.refcount -= 1;
            }
        }
    }
}

impl Drop for ZendString {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}
