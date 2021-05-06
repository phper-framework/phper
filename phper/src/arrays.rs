//! Apis relate to [crate::sys::zend_array].

use crate::{
    alloc::{EAllocatable, EBox},
    sys::*,
    values::Val,
};
use std::mem::zeroed;

/// Key for [Array].
pub enum Key<'a> {
    Index(u64),
    Str(&'a str),
}

impl From<u64> for Key<'_> {
    fn from(i: u64) -> Self {
        Key::Index(i)
    }
}

impl<'a> From<&'a str> for Key<'a> {
    fn from(s: &'a str) -> Self {
        Key::Str(s)
    }
}

/// Wrapper of [crate::sys::zend_array].
#[repr(transparent)]
pub struct Array {
    inner: zend_array,
}

impl Array {
    // TODO Change to EBox<Self>.
    pub fn new() -> Self {
        unsafe {
            let mut array = zeroed::<Array>();
            _zend_hash_init(array.as_mut_ptr(), 0, None, false.into());
            array
        }
    }

    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Array {
        let ptr = ptr as *mut Array;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    pub fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    pub fn insert<'a>(&mut self, key: impl Into<Key<'a>>, value: Val) {
        let key = key.into();
        let value = EBox::new(value);
        unsafe {
            match key {
                Key::Index(i) => {
                    zend_hash_index_update(&mut self.inner, i, EBox::into_raw(value).cast());
                }
                Key::Str(s) => {
                    phper_zend_hash_str_update(
                        &mut self.inner,
                        s.as_ptr().cast(),
                        s.len(),
                        EBox::into_raw(value).cast(),
                    );
                }
            }
        }
    }

    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&Val> {
        let key = key.into();
        unsafe {
            let value = match key {
                Key::Index(i) => zend_hash_index_find(&self.inner, i),
                Key::Str(s) => zend_hash_str_find(&self.inner, s.as_ptr().cast(), s.len()),
            };
            if value.is_null() {
                None
            } else {
                Some(Val::from_mut_ptr(value))
            }
        }
    }

    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(&mut self.inner) as usize }
    }
}

impl EAllocatable for Array {
    fn free(ptr: *mut Self) {
        unsafe {
            zend_hash_destroy(ptr.cast());
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            zend_hash_destroy(&mut self.inner);
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        let mut other = Self::new();
        unsafe {
            zend_hash_copy(other.as_mut_ptr(), self.as_ptr() as *mut _, None);
        }
        other
    }
}
