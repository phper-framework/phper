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
    pub fn new(s: impl AsRef<[u8]>) -> EBox<Self> {
        unsafe {
            let s = s.as_ref();
            let ptr = phper_zend_string_init(s.as_ptr().cast(), s.len(), false.into()).cast();
            EBox::from_raw(ptr)
        }
    }

    pub unsafe fn from_raw(ptr: *mut zend_string) -> EBox<Self> {
        EBox::from_raw(ptr as *mut ZendString)
    }

    pub unsafe fn from_ptr<'a>(ptr: *mut zend_string) -> Option<&'a Self> {
        let ptr = ptr as *mut Self;
        ptr.as_ref()
    }

    pub fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_ref())
    }
}

impl AsRef<[u8]> for ZendString {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                &self.inner.val as *const c_char as *const u8,
                self.inner.len,
            )
        }
    }
}

impl<Rhs: AsRef<[u8]>> PartialEq<Rhs> for ZendString {
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl EAllocatable for ZendString {
    fn free(ptr: *mut Self) {
        unsafe {
            // Already has `GC_DELREF(s) == 0` detection.
            phper_zend_string_release(ptr.cast());
        }
    }
}

impl Drop for ZendString {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}
