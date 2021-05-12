//! Apis relate to [crate::sys::zend_object].

use std::{marker::PhantomData, ptr::null_mut};
use std::any::Any;
use std::ffi::c_void;
use std::hash::Hasher;
use std::io::Write;
use std::mem::{size_of, ManuallyDrop};
use std::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{
    alloc::{EAllocatable, EBox},
    classes::ClassEntry,
    errors::ClassNotFoundError,
    sys::*,
    values::Val,
};

/// Wrapper of [crate::sys::zend_object].
#[repr(transparent)]
pub struct Object<T> {
    inner: zend_object,
    _p: PhantomData<T>,
}

impl<T> Object<T> {
    pub fn new(class_entry: &ClassEntry) -> EBox<Self> {
        unsafe {
            let ptr = zend_objects_new(class_entry.as_ptr() as *mut _);
            EBox::from_raw(ptr.cast())
        }
    }

    pub fn new_by_class_name(
        class_name: impl AsRef<str>,
    ) -> Result<EBox<Self>, ClassNotFoundError> {
        let class_entry = ClassEntry::from_globals(class_name)?;
        Ok(Self::new(class_entry))
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_object) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should not be null")
    }

    pub fn as_ptr(&self) -> *const zend_object {
        &self.inner
    }

    pub fn as_mut_ptr(&mut self) -> *mut zend_object {
        &mut self.inner
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> &Val {
        let name = name.as_ref();

        let prop = unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    self.inner.ce,
                    &self.inner as *const _ as *mut _,
                    name.as_ptr().cast(),
                    name.len(),
                    true.into(),
                    null_mut(),
                )
            }
            #[cfg(phper_major_version = "7")]
            {
                let mut zv = std::mem::zeroed::<zval>();
                phper_zval_obj(&mut zv, self.as_ptr() as *mut _);
                zend_read_property(
                    self.inner.ce,
                    &mut zv,
                    name.as_ptr().cast(),
                    name.len(),
                    true.into(),
                    null_mut(),
                )
            }
        };

        unsafe { Val::from_mut_ptr(prop) }
    }

    pub fn set_property(&mut self, name: impl AsRef<str>, val: Val) {
        let name = name.as_ref();
        let val = EBox::new(val);
        unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_update_property(
                    self.inner.ce,
                    &mut self.inner,
                    name.as_ptr().cast(),
                    name.len(),
                    EBox::into_raw(val).cast(),
                )
            }
            #[cfg(phper_major_version = "7")]
            {
                let mut zv = std::mem::zeroed::<zval>();
                phper_zval_obj(&mut zv, self.as_mut_ptr());
                zend_update_property(
                    self.inner.ce,
                    &mut zv,
                    name.as_ptr().cast(),
                    name.len(),
                    EBox::into_raw(val).cast(),
                )
            }
        }
    }

    pub fn clone_obj(&self) -> EBox<Self> {
        unsafe {
            let new_obj = {
                #[cfg(phper_major_version = "8")]
                {
                    zend_objects_clone_obj(self.as_ptr() as *mut _).cast()
                }
                #[cfg(phper_major_version = "7")]
                {
                    let mut zv = std::mem::zeroed::<zval>();
                    phper_zval_obj(&mut zv, self.as_ptr() as *mut _);
                    zend_objects_clone_obj(&mut zv).cast()
                }
            };

            EBox::from_raw(new_obj)
        }
    }
}

impl Object<()> {
    pub fn new_by_std_class() -> EBox<Self> {
        Self::new_by_class_name("stdclass").unwrap()
    }
}

impl<T> EAllocatable for Object<T> {
    fn free(ptr: *mut Self) {
        unsafe {
            zend_objects_destroy_object(ptr.cast());
        }
    }
}

impl<T> Drop for Object<T> {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}

/// The Object contains `zend_object` and the user defined state data.
#[repr(C)]
pub struct ExtendObject {
    pub(crate) state: ManuallyDrop<Box<dyn Any>>,
    pub(crate) object: zend_object,
}

impl ExtendObject {
    pub(crate) const fn offset() -> usize {
       size_of::<ManuallyDrop<Box<dyn Any>>>()
    }
}

