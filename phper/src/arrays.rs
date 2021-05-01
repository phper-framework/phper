use crate::{sys::*, values::Val};
use std::mem::zeroed;

#[repr(transparent)]
pub struct Array {
    inner: zend_array,
}

impl Array {
    pub fn new() -> Self {
        unsafe {
            let mut array = zeroed::<Array>();
            _zend_hash_init(&mut array.inner, 0, None, true.into());
            array
        }
    }

    pub(crate) unsafe fn from_raw<'a>(ptr: *mut zend_array) -> &'a mut Array {
        let ptr = ptr as *mut Array;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    pub fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    pub fn insert(&mut self, key: impl AsRef<str>, mut value: Val) {
        let key = key.as_ref();
        unsafe {
            phper_zend_hash_str_update(
                &mut self.inner,
                key.as_ptr().cast(),
                key.len(),
                value.as_mut_ptr(),
            );
        }
    }

    pub fn get(&mut self, key: impl AsRef<str>) -> &mut Val {
        let key = key.as_ref();
        unsafe {
            let value = zend_hash_str_find(&mut self.inner, key.as_ptr().cast(), key.len());
            Val::from_mut(value)
        }
    }

    pub fn len(&mut self) -> usize {
        unsafe { zend_array_count(&mut self.inner) as usize }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            zend_hash_destroy(&mut self.inner);
        }
    }
}
