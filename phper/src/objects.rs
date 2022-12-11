// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_object].

use crate::{
    alloc::EBox,
    classes::ClassEntry,
    functions::{call_internal, ZendFunction},
    sys::*,
    values::ZVal,
};
use phper_alloc::ToRefOwned;
use std::{
    any::Any,
    borrow::Borrow,
    convert::TryInto,
    fmt::{self, Debug},
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
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_object) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_object) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_object) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_object) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    pub const fn as_ptr(&self) -> *const zend_object {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_object {
        &mut self.inner
    }

    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_stateful_obj<T: 'static>(&self) -> &StateObj<T> {
        transmute(self)
    }

    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_mut_stateful_obj<T: 'static>(&mut self) -> &mut StateObj<T> {
        transmute(self)
    }

    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_state<T: 'static>(&self) -> &T {
        let eo = StateObject::fetch(&self.inner);
        eo.state.downcast_ref().unwrap()
    }

    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_mut_state<T: 'static>(&mut self) -> &mut T {
        let eo = StateObject::fetch_mut(&mut self.inner);
        eo.state.downcast_mut().unwrap()
    }

    #[inline]
    pub fn handle(&self) -> u32 {
        self.inner.handle
    }

    pub fn get_class(&self) -> &ClassEntry {
        unsafe { ClassEntry::from_ptr(self.inner.ce) }
    }

    pub fn get_property(&self, name: impl AsRef<str>) -> &ZVal {
        let object = self.as_ptr() as *mut _;
        let prop = Self::inner_get_property(self.inner.ce, object, name);
        unsafe { ZVal::from_ptr(prop) }
    }

    pub fn get_mut_property(&mut self, name: impl AsRef<str>) -> &mut ZVal {
        let object = self.as_mut_ptr();
        let prop = Self::inner_get_property(self.inner.ce, object, name);
        unsafe { ZVal::from_mut_ptr(prop) }
    }

    #[allow(clippy::useless_conversion)]
    fn inner_get_property(
        scope: *mut zend_class_entry, object: *mut zend_object, name: impl AsRef<str>,
    ) -> *mut zval {
        let name = name.as_ref();

        unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_read_property(
                    scope,
                    object,
                    name.as_ptr().cast(),
                    name.len().try_into().unwrap(),
                    true.into(),
                    null_mut(),
                )
            }
            #[cfg(phper_major_version = "7")]
            {
                let mut zv = std::mem::zeroed::<zval>();
                phper_zval_obj(&mut zv, object);
                zend_read_property(
                    scope,
                    &mut zv,
                    name.as_ptr().cast(),
                    name.len().try_into().unwrap(),
                    true.into(),
                    null_mut(),
                )
            }
        }
    }

    #[allow(clippy::useless_conversion)]
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
    /// use phper::{alloc::EBox, classes::ClassEntry, values::ZVal};
    ///
    /// fn example() -> phper::Result<ZVal> {
    ///     let mut memcached = ClassEntry::from_globals("Memcached")?.new_object(&mut [])?;
    ///     memcached.call(
    ///         "addServer",
    ///         &mut [ZVal::from("127.0.0.1"), ZVal::from(11211)],
    ///     )?;
    ///     let r = memcached.call("get", &mut [ZVal::from("hello")])?;
    ///     Ok(r)
    /// }
    /// ```
    pub fn call(
        &mut self, method_name: &str, arguments: impl AsMut<[ZVal]>,
    ) -> crate::Result<ZVal> {
        let mut method = method_name.into();
        call_internal(&mut method, Some(self), arguments)
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

impl Debug for ZObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZObj")
            .field("class", &self.get_class().get_name().to_c_str())
            .field("handle", &self.handle())
            .finish()
    }
}

/// Wrapper of [crate::sys::zend_object].
pub struct ZObject {
    inner: *mut ZObj,
}

impl ZObject {
    /// Another way to new object like [crate::classes::ClassEntry::new_object].
    pub fn new(class_entry: &ClassEntry, arguments: impl AsMut<[ZVal]>) -> crate::Result<Self> {
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

    /// Create owned object From raw pointer, usually used in pairs with
    /// `into_raw`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example, a double-free may occur if the function is called
    /// twice on the same raw pointer.
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
            Self::from_raw({
                let mut zv = ManuallyDrop::new(ZVal::default());
                phper_zval_obj(zv.as_mut_ptr(), self.as_ptr() as *mut _);
                let handlers = phper_z_obj_ht_p(zv.as_ptr());

                let ptr = {
                    #[cfg(phper_major_version = "7")]
                    {
                        zv.as_mut_ptr()
                    }
                    #[cfg(phper_major_version = "8")]
                    {
                        self.as_ptr() as *mut _
                    }
                };

                match (*handlers).clone_obj {
                    Some(clone_obj) => clone_obj(ptr),
                    None => zend_objects_clone_obj(ptr),
                }
            })
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

impl Debug for ZObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZObject")
            .field("class", &self.get_class().get_name().to_c_str())
            .field("handle", &self.handle())
            .finish()
    }
}

#[repr(transparent)]
pub struct StateObj<T> {
    inner: ZObj,
    _p: PhantomData<T>,
}

impl<T: 'static> StateObj<T> {
    pub fn as_state(&self) -> &T {
        unsafe { self.inner.as_state() }
    }

    pub fn as_mut_state(&mut self) -> &mut T {
        unsafe { self.inner.as_mut_state() }
    }
}

impl<T> Deref for StateObj<T> {
    type Target = ZObj;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for StateObj<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub(crate) type ManuallyDropState = ManuallyDrop<Box<dyn Any>>;

/// The Object contains `zend_object` and the user defined state data.
#[repr(C)]
pub(crate) struct StateObject {
    state: ManuallyDropState,
    object: zend_object,
}

impl StateObject {
    pub(crate) const fn offset() -> usize {
        size_of::<ManuallyDropState>()
    }

    pub(crate) unsafe fn fetch(object: &zend_object) -> &Self {
        (((object as *const _ as usize) - StateObject::offset()) as *const Self)
            .as_ref()
            .unwrap()
    }

    pub(crate) unsafe fn fetch_mut(object: &mut zend_object) -> &mut Self {
        (((object as *mut _ as usize) - StateObject::offset()) as *mut Self)
            .as_mut()
            .unwrap()
    }

    pub(crate) fn fetch_ptr(object: *mut zend_object) -> *mut Self {
        (object as usize - StateObject::offset()) as *mut Self
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
