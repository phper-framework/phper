use crate::{
    classes::ClassEntry,
    sys::*,
    values::{SetVal, Val},
    ClassNotFoundError,
};
use phper_alloc::EBox;
use std::{
    mem::{forget, zeroed},
    ptr::null_mut,
};

/// Wrapper of [crate::sys::zend_object].
#[repr(transparent)]
pub struct Object {
    inner: zend_object,
}

impl Object {
    pub fn new(class_entry: &ClassEntry) -> Self {
        unsafe {
            let mut object = zeroed::<Object>();
            zend_object_std_init(object.as_mut_ptr(), class_entry.as_ptr() as *mut _);
            object.inner.handlers = &std_object_handlers;
            object
        }
    }

    pub fn new_by_class_name(class_name: impl AsRef<str>) -> Result<Self, ClassNotFoundError> {
        let class_entry = ClassEntry::from_globals(class_name)?;
        Ok(Self::new(class_entry))
    }

    pub fn new_by_std_class() -> Self {
        Self::new_by_class_name("stdclass").unwrap()
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
                let mut zv = zeroed::<zval>();
                phper_zval_obj(&mut zv, self.as_ptr() as *mut _);
                zend_read_property(
                    self.inner.ce,
                    &mut zv,
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
            #[cfg(phper_major_version = "8")]
            {
                zend_update_property(
                    self.inner.ce,
                    &mut self.inner,
                    name.as_ptr().cast(),
                    name.len(),
                    val.as_mut_ptr(),
                )
            }
            #[cfg(phper_major_version = "7")]
            {
                let mut zv = zeroed::<zval>();
                phper_zval_obj(&mut zv, self.as_mut_ptr());
                zend_update_property(
                    self.inner.ce,
                    &mut zv,
                    name.as_ptr().cast(),
                    name.len(),
                    val.as_mut_ptr(),
                )
            }
        }
    }

    pub fn clone_obj(&self) -> EBox<Self> {
        unsafe {
            let new_obj = zend_objects_clone_obj(self.as_ptr() as *mut _).cast();
            EBox::from_raw(new_obj)
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
