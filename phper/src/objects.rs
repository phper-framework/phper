use crate::{classes::ClassEntry, sys::*, values::Val};
use std::ptr::null_mut;

pub struct Object {
    val: *mut Val,
    class: *mut ClassEntry,
}

impl Object {
    pub(crate) fn new<'a>(val: *mut Val, class: *mut ClassEntry) -> Object {
        assert!(!val.is_null());
        assert!(!class.is_null());
        Self { val, class }
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> &mut Val {
        let name = name.as_ref();

        let prop = unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    self.class as *mut _,
                    (*self.val).inner.value.obj,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }

            #[cfg(phper_major_version = "7")]
            {
                zend_read_property(
                    self.class as *mut _,
                    self.val as *mut _,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }
        };

        unsafe { Val::from_mut(prop) }
    }
}
