//! Apis relate to [crate::sys::zend_object].

use crate::{
    alloc::{EAllocatable, EBox},
    classes::ClassEntry,
    errors::ClassNotFoundError,
    sys::*,
    values::Val,
};
use std::{marker::PhantomData, ptr::null_mut};
use std::mem::size_of;
use std::slice::{from_raw_parts_mut, from_raw_parts};
use std::hash::Hasher;
use std::io::Write;
use std::ffi::c_void;

/// Wrapper of [crate::sys::zend_object].
#[repr(transparent)]
pub struct Object<T> {
    inner: zend_object,
    _p: PhantomData<T>,
}

impl<T> Object<T> {
    pub fn new(class_entry: &ClassEntry, constructor: impl FnOnce() -> T) -> EBox<Self> {
        // Like `zend_objects_new`, but `emalloc` more size to store `T`.
        unsafe {
            let ce = class_entry.as_ptr() as *mut _;
            let ori_len = size_of::<zend_object>() + phper_zend_object_properties_size(ce);
            let total_len = ori_len + size_of::<*mut T>();
            let object = _emalloc(total_len).cast();
            zend_object_std_init(object, ce);
            (*object).handlers = &std_object_handlers;

            let t = Box::new(constructor());
            let ptr = Box::into_raw(t) as usize;

            let data = from_raw_parts_mut(object as *mut u8, total_len);
            let mut data = &mut data[ori_len..];
            data.write_all(&ptr.to_le_bytes());

            EBox::from_raw(object.cast())
        }
    }

    pub fn new_by_class_name(
        class_name: impl AsRef<str>,
        constructor: impl FnOnce() -> T,
    ) -> Result<EBox<Self>, ClassNotFoundError> {
        let class_entry = ClassEntry::from_globals(class_name)?;
        Ok(Self::new(class_entry, constructor))
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
        Self::new_by_class_name("stdclass", || ()).unwrap()
    }
}

impl<T> EAllocatable for Object<T> {
    fn free(ptr: *mut Self) {
        unsafe {
            hack_zend_objects_destroy_object(ptr.cast());
        }
    }
}

impl<T> Drop for Object<T> {
    fn drop(&mut self) {
        unreachable!("Allocation on the stack is not allowed")
    }
}

unsafe extern "C" fn hack_zend_objects_destroy_object(object: *mut zend_object) {
    let ce = (*object).ce;
    let ori_len = size_of::<zend_object>() + phper_zend_object_properties_size(ce);
    let total_len = ori_len + size_of::<*mut c_void>();

    let data = from_raw_parts(object as *mut u8, total_len);
    let data = &data[ori_len..];

    let mut buf = [0u8; 8];
    (&mut buf[..]).write_all(data);
    let ptr = usize::from_le_bytes(buf) as *mut c_void;

    // TODO why to find the type of `T` ???

    zend_objects_destroy_object(object);
}