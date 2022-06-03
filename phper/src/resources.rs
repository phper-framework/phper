//! Apis relate to [crate::sys::zend_resource].

use crate::sys::*;

/// Wrapper of [crate::sys::zend_resource].
#[repr(transparent)]
pub struct Resource {
    inner: zend_resource,
}

impl Resource {
    /// # Safety
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_resource) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should not be null")
    }

    #[allow(clippy::useless_conversion)]
    pub fn handle(&self) -> i64 {
        self.inner.handle.into()
    }
}
