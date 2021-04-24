use crate::{sys::*, values::Val};
use std::mem::size_of;

pub struct Array {
    inner: *mut zend_array,
    leak: bool,
}

impl Array {
    pub fn new() -> Self {
        unsafe {
            let inner = _emalloc(size_of::<zend_array>()).cast();
            _zend_hash_init(inner, 0, None, true.into());
            Self { inner, leak: false }
        }
    }

    pub fn as_ptr(&self) -> *const zend_array {
        self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        self.inner
    }

    pub fn insert(&mut self, key: impl AsRef<str>, value: &mut Val) {
        let key = key.as_ref();
        unsafe {
            phper_zend_hash_str_update(self.inner, key.as_ptr().cast(), key.len(), value.as_mut());
        }
    }

    pub fn get(&mut self, key: impl AsRef<str>) -> &mut Val {
        let key = key.as_ref();
        unsafe {
            let value = zend_hash_str_find(self.inner, key.as_ptr().cast(), key.len());
            Val::from_mut(value)
        }
    }

    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(self.inner) as usize }
    }

    pub(crate) fn leak(&mut self) -> &mut bool {
        &mut self.leak
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            if !self.leak {
                zend_hash_destroy(self.inner);
                _efree(self.inner.cast());
            }
        }
    }
}
