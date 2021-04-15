use crate::sys::*;

pub struct ZString {
    inner: *mut zend_string,
}

impl ZString {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner: phper_zend_string_alloc(0, 1),
            }
        }
    }
}

impl<T: AsRef<str>> From<T> for ZString {
    fn from(t: T) -> Self {
        let s = t.as_ref();
        unsafe {
            Self {
                inner: phper_zend_string_init(s.as_ptr().cast(), s.len(), 1),
            }
        }
    }
}

impl Drop for ZString {
    fn drop(&mut self) {
        unsafe {
            phper_zend_string_release(self.inner);
        }
    }
}