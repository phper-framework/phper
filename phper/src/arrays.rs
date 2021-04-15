use crate::sys::*;
use std::ptr::null_mut;
use std::mem::zeroed;
use crate::values::Val;
use std::ops::{Deref, DerefMut};

pub struct Array {
    inner: Box<zend_array>,
}

impl Array {
    pub fn new() -> Self {
        let mut inner = Box::new(unsafe { zeroed::<zend_array>() });
        unsafe {
            _zend_hash_init(&mut *inner, 0, None, 1);
        }
        Self {
            inner,
        }
    }

    pub fn insert(&mut self, key: impl AsRef<str>, value: &mut Val) {
        let key = key.as_ref();
        unsafe {
            zend_hash_str_update(&mut *self.inner, key.as_ptr().cast(), key.len(), value.as_mut());
        }
    }

    pub fn get(&mut self, key: impl AsRef<str>) -> &mut Val {
        let key = key.as_ref();
        unsafe {
            let value = zend_hash_str_find(&mut *self.inner, key.as_ptr().cast(), key.len());
            Val::from_mut(value)
        }
    }

    pub fn len(&mut self) -> usize {
        unsafe {
            zend_array_count(&mut *self.inner) as usize
        }
    }
}

impl AsRef<zend_array> for Array {
    fn as_ref(&self) -> &zend_array {
        self.inner.deref()
    }
}

impl AsMut<zend_array> for Array {
    fn as_mut(&mut self) -> &mut zend_array {
        self.inner.deref_mut()
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            zend_hash_destroy(&mut *self.inner);
        }
    }
}