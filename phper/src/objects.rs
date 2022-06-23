// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_object].

use phper_alloc::ToRefOwned;

use crate::{
    alloc::EBox,
    classes::ClassEntry,
    functions::{call_internal, ZendFunction},
    sys::*,
    values::ZVal,
};
use std::{
    any::Any,
    borrow::Borrow,
    convert::TryInto,
    intrinsics::transmute,
    marker::PhantomData,
    mem::{forget, size_of, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

/// Wrapper of [crate::sys::zend_object].
#[repr(transparent)]
pub struct ZObj {
    inner: zend_object,
    _p: PhantomData<*mut ()>,
}

impl ZObj {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_object) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_object) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    pub const fn as_ptr(&self) -> *const zend_object {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_object {
        &mut self.inner
    }

    pub unsafe fn as_state<T: 'static>(&self) -> &T {
        let eo = ExtendObject::fetch(&self.inner);
        eo.state.downcast_ref().unwrap()
    }

    pub unsafe fn as_mut_state<T: 'static>(&mut self) -> &mut T {
        let eo = ExtendObject::fetch_mut(&mut self.inner);
        eo.state.downcast_mut().unwrap()
    }

    pub fn get_class(&self) -> &ClassEntry {
        unsafe { ClassEntry::from_ptr(self.inner.ce) }
    }

    pub fn get_property(&mut self, name: impl AsRef<str>) -> &ZVal {
        self.get_mut_property(name)
    }

    #[allow(clippy::useless_conversion)]
    pub fn get_mut_property(&mut self, name: impl AsRef<str>) -> &mut ZVal {
        let name = name.as_ref();

        let prop = unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    self.inner.ce,
                    &self.inner as *const _ as *mut _,
                    name.as_ptr().cast(),
                    name.len().try_into().unwrap(),
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
                    name.len().try_into().unwrap(),
                    true.into(),
                    null_mut(),
                )
            }
        };

        unsafe { ZVal::from_mut_ptr(prop) }
    }

    pub fn set_property(&mut self, name: impl AsRef<str>, val: impl Into<ZVal>) {
        let name = name.as_ref();
        let val = EBox::new(val.into());
        unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_update_property(
                    self.inner.ce,
                    &mut self.inner,
                    name.as_ptr().cast(),
                    name.len().try_into().unwrap(),
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
                    name.len().try_into().unwrap(),
                    EBox::into_raw(val).cast(),
                )
            }
        }
    }

    /// Call the object method by name.
    ///
    /// # Examples
    ///
    /// ```
    /// use phper::{alloc::EBox, classes::StatelessClassEntry, values::Val};
    ///
    /// fn example() -> phper::Result<EBox<Val>> {
    ///     let mut memcached = StatelessClassEntry::from_globals("Memcached")?.new_object(&mut [])?;
    ///     memcached.call("addServer", &mut [Val::new("127.0.0.1"), Val::new(11211)])?;
    ///     let r = memcached.call("get", &mut [Val::new("hello")])?;
    ///     Ok(r)
    /// }
    /// ```
    pub fn call(
        &mut self, method_name: &str, arguments: impl AsMut<[ZVal]>,
    ) -> crate::Result<EBox<ZVal>> {
        let mut method = method_name.into();

        unsafe {
            let mut val = ZVal::from(());
            phper_zval_obj(val.as_mut_ptr(), self.as_mut_ptr());
            call_internal(&mut method, Some(self), arguments)
        }
    }

    /// Return bool represents whether the constructor exists.
    pub(crate) fn call_construct(&mut self, arguments: impl AsMut<[ZVal]>) -> crate::Result<bool> {
        unsafe {
            match (*self.inner.handlers).get_constructor {
                Some(get_constructor) => {
                    let f = get_constructor(self.as_mut_ptr());
                    if f.is_null() {
                        Ok(false)
                    } else {
                        let zend_fn = ZendFunction::from_mut_ptr(f);
                        zend_fn.call(Some(self), arguments)?;
                        Ok(true)
                    }
                }
                None => Ok(false),
            }
        }
    }
}

impl ToRefOwned for ZObj {
    type Owned = ZObject;

    fn to_ref_owned(&mut self) -> Self::Owned {
        let mut val = ManuallyDrop::new(ZVal::default());
        unsafe {
            phper_zval_obj(val.as_mut_ptr(), self.as_mut_ptr());
            phper_z_addref_p(val.as_mut_ptr());
            ZObject::from_raw(val.as_mut_z_obj().unwrap().as_mut_ptr())
        }
    }
}

/// Wrapper of [crate::sys::zend_object].
pub struct ZObject {
    inner: *mut ZObj,
}

impl ZObject {
    /// Another way to new object like [crate::classes::ClassEntry::new_object].
    pub fn new(class_entry: &ClassEntry, arguments: &mut [ZVal]) -> crate::Result<Self> {
        class_entry.new_object(arguments)
    }

    pub fn new_by_class_name(
        class_name: impl AsRef<str>, arguments: &mut [ZVal],
    ) -> crate::Result<Self> {
        let class_entry = ClassEntry::from_globals(class_name)?;
        Self::new(class_entry, arguments)
    }

    pub fn new_by_std_class() -> Self {
        Self::new_by_class_name("stdclass", &mut []).unwrap()
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut zend_object) -> Self {
        Self {
            inner: ZObj::from_mut_ptr(ptr),
        }
    }

    #[inline]
    pub fn into_raw(mut self) -> *mut zend_object {
        let ptr = self.as_mut_ptr();
        forget(self);
        ptr
    }
}

impl Clone for ZObject {
    fn clone(&self) -> Self {
        unsafe {
            // TODO Get the handle clone_obj of object.
            let ptr = {
                #[cfg(phper_major_version = "7")]
                {
                    let mut zv = ZVal::default();
                    phper_zval_obj(zv.as_mut_ptr(), self.as_ptr() as *mut _);
                    let handlers = phper_z_obj_ht_p(zv.as_ptr());
                    match (*handlers).clone_obj {
                        Some(clone_obj) => clone_obj(zv.as_mut_ptr()),
                        None => zend_objects_clone_obj(zv.as_mut_ptr()),
                    }
                }
                #[cfg(phper_major_version = "8")]
                {
                    // zend_objects_clone_obj(self.as_ptr() as *mut _).cast()
                    todo!()
                }
            };
            Self::from_raw(ptr)
        }
    }
}

impl Deref for ZObject {
    type Target = ZObj;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl DerefMut for ZObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap() }
    }
}

impl Borrow<ZObj> for ZObject {
    fn borrow(&self) -> &ZObj {
        self.deref()
    }
}

impl Drop for ZObject {
    fn drop(&mut self) {
        unsafe {
            phper_zend_object_release(self.as_mut_ptr());
        }
    }
}

#[repr(transparent)]
pub struct StatefulObj<T> {
    inner: ZObj,
    _p: PhantomData<T>,
}

impl<T: 'static> StatefulObj<T> {
    pub unsafe fn from_z_obj(obj: &mut ZObj) -> &mut Self {
        transmute(obj)
    }

    pub fn as_state(&self) -> &T {
        unsafe { self.inner.as_state() }
    }

    pub fn as_mut_state(&mut self) -> &mut T {
        unsafe { self.inner.as_mut_state() }
    }
}

impl<T> Deref for StatefulObj<T> {
    type Target = ZObj;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for StatefulObj<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub(crate) type ManuallyDropState = ManuallyDrop<Box<dyn Any>>;

/// The Object contains `zend_object` and the user defined state data.
#[repr(C)]
pub(crate) struct ExtendObject {
    state: ManuallyDropState,
    object: zend_object,
}

impl ExtendObject {
    pub(crate) const fn offset() -> usize {
        size_of::<ManuallyDropState>()
    }

    pub(crate) fn fetch(object: &zend_object) -> &Self {
        unsafe {
            (((object as *const _ as usize) - ExtendObject::offset()) as *const Self)
                .as_ref()
                .unwrap()
        }
    }

    pub(crate) fn fetch_mut(object: &mut zend_object) -> &mut Self {
        unsafe {
            (((object as *mut _ as usize) - ExtendObject::offset()) as *mut Self)
                .as_mut()
                .unwrap()
        }
    }

    pub(crate) fn fetch_ptr(object: *mut zend_object) -> *mut Self {
        (object as usize - ExtendObject::offset()) as *mut Self
    }

    pub(crate) unsafe fn drop_state(this: *mut Self) {
        let state = &mut (*this).state;
        ManuallyDrop::drop(state);
    }

    pub(crate) unsafe fn as_mut_state<'a>(this: *mut Self) -> &'a mut ManuallyDropState {
        &mut (*this).state
    }

    pub(crate) unsafe fn as_mut_object<'a>(this: *mut Self) -> &'a mut zend_object {
        &mut (*this).object
    }
}
