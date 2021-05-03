use crate::sys::*;

/// Wrapper of [crate::sys::zend_string].
#[repr(transparent)]
pub struct ZendString {
    inner: zend_string,
}

impl ZendString {
    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Self {
        let ptr = ptr as *mut Self;
        ptr.as_mut().expect("ptr shouldn't be null")
    }

    pub fn as_ptr(&self) -> *const zend_string {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_string {
        &mut self.inner
    }
}
