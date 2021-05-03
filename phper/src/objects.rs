use crate::{
    classes::get_global_class_entry_ptr,
    sys::*,
    values::{SetVal, Val},
    ClassNotFoundError,
};
use std::{mem::zeroed, ptr::null_mut};

/// Wrapper of [crate::sys::zend_object].
#[repr(transparent)]
pub struct Object {
    inner: zend_object,
}

impl Object {
    pub fn new(class_name: impl AsRef<str>) -> Result<Self, ClassNotFoundError> {
        unsafe {
            let mut object = zeroed::<Object>();
            let class_name = class_name.as_ref();
            let ce = get_global_class_entry_ptr(class_name);
            if ce.is_null() {
                Err(ClassNotFoundError::new(class_name.to_string()))
            } else {
                zend_object_std_init(object.as_mut_ptr(), ce);
                object.inner.handlers = &std_object_handlers;
                Ok(object)
            }
        }
    }

    pub fn new_std() -> Self {
        Self::new("stdClass").expect("stdClass not found")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_object) -> &'a mut Object {
        (ptr as *mut Object)
            .as_mut()
            .expect("ptr should not be null")
    }

    pub fn as_ptr(&self) -> *const zend_object {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_object {
        &mut self.inner
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> &mut Val {
        let name = name.as_ref();

        let prop = unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    self.inner.ce,
                    &self.inner as *const _ as *mut _,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }

            #[cfg(phper_major_version = "7")]
            {
                zend_read_property(
                    self.inner.ce,
                    &self.inner as *const _ as *mut _,
                    name.as_ptr().cast(),
                    name.len(),
                    false.into(),
                    null_mut(),
                )
            }
        };

        unsafe { Val::from_mut_ptr(prop) }
    }

    pub fn set_property(&mut self, name: impl AsRef<str>, value: impl SetVal) {
        let name = name.as_ref();
        let mut val = Val::new(value);
        unsafe {
            zend_update_property(
                self.inner.ce,
                &mut self.inner as *mut _,
                name.as_ptr().cast(),
                name.len(),
                val.as_mut_ptr(),
            )
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            zend_objects_destroy_object(&mut self.inner);
        }
    }
}
