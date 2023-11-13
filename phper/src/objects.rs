// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_object].

use crate::{
    classes::ClassEntry,
    functions::{call_internal, call_raw_common, ZFunc},
    sys::*,
    values::ZVal,
};
use phper_alloc::{RefClone, ToRefOwned};
use std::{
    any::Any,
    borrow::Borrow,
    convert::TryInto,
    ffi::c_void,
    fmt::{self, Debug},
    marker::PhantomData,
    mem::{replace, size_of, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

/// Wrapper of [zend_object].
#[repr(transparent)]
pub struct ZObj {
    inner: zend_object,
    _p: PhantomData<*mut ()>,
}

impl ZObj {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_object) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_object) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_object) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_object) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_object {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_object {
        &mut self.inner
    }

    /// Upgrade to state obj.
    ///
    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_state_obj<T>(&self) -> &StateObj<T> {
        StateObj::from_object_ptr(self.as_ptr())
    }

    /// Upgrade to mutable state obj.
    ///
    /// # Safety
    ///
    /// Should only call this method for the class of object defined by the
    /// extension created by `phper`, otherwise, memory problems will caused.
    pub unsafe fn as_mut_state_obj<T>(&mut self) -> &mut StateObj<T> {
        StateObj::from_mut_object_ptr(self.as_mut_ptr())
    }

    /// Get the inner handle of object.
    #[inline]
    pub fn handle(&self) -> u32 {
        self.inner.handle
    }

    /// Get the class reference of object.
    pub fn get_class(&self) -> &ClassEntry {
        unsafe { ClassEntry::from_ptr(self.inner.ce) }
    }

    /// Get the mutable class reference of object.
    pub fn get_mut_class(&mut self) -> &mut ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.inner.ce) }
    }

    /// Get the property by name of object.
    pub fn get_property(&self, name: impl AsRef<str>) -> &ZVal {
        let object = self.as_ptr() as *mut _;
        let prop = Self::inner_get_property(self.inner.ce, object, name);
        unsafe { ZVal::from_ptr(prop) }
    }

    /// Get the mutable property by name of object.
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

    /// Set the property by name of object.
    #[allow(clippy::useless_conversion)]
    pub fn set_property(&mut self, name: impl AsRef<str>, val: impl Into<ZVal>) {
        let name = name.as_ref();
        let mut val = val.into();
        unsafe {
            #[cfg(phper_major_version = "8")]
            {
                zend_update_property(
                    self.inner.ce,
                    &mut self.inner,
                    name.as_ptr().cast(),
                    name.len().try_into().unwrap(),
                    val.as_mut_ptr(),
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
                    val.as_mut_ptr(),
                )
            }
        }
    }

    /// Call the object method by name.
    ///
    /// # Examples
    ///
    /// ```no_run
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

    pub(crate) fn call_construct(&mut self, arguments: impl AsMut<[ZVal]>) -> crate::Result<()> {
        unsafe {
            let Some(get_constructor) = (*self.inner.handlers).get_constructor else {
                return Ok(());
            };

            // The `get_constructor` is possible to throw PHP Error, so call it inside
            // `call_raw_common`.
            let mut val = call_raw_common(|val| {
                let f = get_constructor(self.as_mut_ptr());
                if !f.is_null() {
                    phper_zval_func(val.as_mut_ptr(), f);
                }
            })?;

            if val.get_type_info().is_null() {
                return Ok(());
            }

            let f = phper_z_func_p(val.as_mut_ptr());
            let zend_fn = ZFunc::from_mut_ptr(f);
            zend_fn.call(Some(self), arguments)?;

            Ok(())
        }
    }

    pub(crate) unsafe fn gc_refcount(&self) -> u32 {
        phper_zend_object_gc_refcount(self.as_ptr())
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
        common_fmt(self, f, "ZObj")
    }
}

/// Wrapper of [zend_object].
pub struct ZObject {
    inner: *mut ZObj,
}

impl ZObject {
    /// Another way to new object like [crate::classes::ClassEntry::new_object].
    pub fn new(class_entry: &ClassEntry, arguments: impl AsMut<[ZVal]>) -> crate::Result<Self> {
        class_entry.new_object(arguments)
    }

    /// New object, like `new`, but get class by [`ClassEntry::from_globals`].
    pub fn new_by_class_name(
        class_name: impl AsRef<str>, arguments: &mut [ZVal],
    ) -> crate::Result<Self> {
        let class_entry = ClassEntry::from_globals(class_name)?;
        Self::new(class_entry, arguments)
    }

    /// New object with class `stdClass`.
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

    /// Consumes and returning a wrapped raw pointer.
    #[inline]
    pub fn into_raw(self) -> *mut zend_object {
        ManuallyDrop::new(self).as_mut_ptr()
    }
}

impl RefClone for ZObject {
    #[inline]
    fn ref_clone(&mut self) -> Self {
        self.to_ref_owned()
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
        common_fmt(self, f, "ZObject")
    }
}

pub(crate) type AnyState = *mut dyn Any;

/// The object owned state, usually as the parameter of method handler.
#[repr(C)]
pub struct StateObj<T> {
    any_state: AnyState,
    object: ZObj,
    _p: PhantomData<T>,
}

impl<T> StateObj<T> {
    /// The `zend_object_alloc` often allocate more memory to hold the state
    /// (usually is a pointer), and place it before `zend_object`.
    pub(crate) const fn offset() -> usize {
        size_of::<AnyState>()
    }

    #[inline]
    pub(crate) unsafe fn from_mut_ptr<'a>(ptr: *mut c_void) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    pub(crate) unsafe fn from_object_ptr<'a>(ptr: *const zend_object) -> &'a Self {
        ((ptr as usize - Self::offset()) as *const Self)
            .as_ref()
            .unwrap()
    }

    pub(crate) unsafe fn from_mut_object_ptr<'a>(ptr: *mut zend_object) -> &'a mut Self {
        ((ptr as usize - Self::offset()) as *mut Self)
            .as_mut()
            .unwrap()
    }

    pub(crate) unsafe fn drop_state(&mut self) {
        drop(Box::from_raw(self.any_state));
    }

    #[inline]
    pub(crate) fn as_mut_any_state(&mut self) -> &mut AnyState {
        &mut self.any_state
    }

    /// Gets object.
    #[inline]
    pub fn as_object(&self) -> &ZObj {
        &self.object
    }

    /// Gets mutable object.
    #[inline]
    pub fn as_mut_object(&mut self) -> &mut ZObj {
        &mut self.object
    }
}

impl<T: 'static> StateObj<T> {
    /// Gets inner state.
    pub fn as_state(&self) -> &T {
        unsafe {
            let any_state = self.any_state.as_ref().unwrap();
            any_state.downcast_ref().unwrap()
        }
    }

    /// Gets inner mutable state.
    pub fn as_mut_state(&mut self) -> &mut T {
        unsafe {
            let any_state = self.any_state.as_mut().unwrap();
            any_state.downcast_mut().unwrap()
        }
    }
}

impl<T> Deref for StateObj<T> {
    type Target = ZObj;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T> DerefMut for StateObj<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<T> Debug for StateObj<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "StateObj")
    }
}

/// The object owned state, usually crated by
/// [StaticStateClass](crate::classes::StaticStateClass).
pub struct StateObject<T> {
    inner: *mut StateObj<T>,
}

impl<T> StateObject<T> {
    #[inline]
    pub(crate) fn from_raw_object(object: *mut zend_object) -> Self {
        unsafe {
            Self {
                inner: StateObj::from_mut_object_ptr(object),
            }
        }
    }

    #[inline]
    pub(crate) fn into_raw_object(self) -> *mut zend_object {
        ManuallyDrop::new(self).as_mut_ptr()
    }

    /// Converts into [ZObject].
    pub fn into_z_object(self) -> ZObject {
        unsafe { ZObject::from_raw(self.into_raw_object()) }
    }
}

impl<T: 'static> StateObject<T> {
    /// Converts into state.
    ///
    /// Because the [zend_object] is refcounted type,
    /// therefore, you can only obtain state ownership when the refcount of the
    /// [zend_object] is `1`, otherwise, it will return
    /// `None`.
    pub fn into_state(mut self) -> Option<T> {
        unsafe {
            if self.gc_refcount() != 1 {
                return None;
            }
            let null: AnyState = Box::into_raw(Box::new(()));
            let ptr = replace(self.as_mut_any_state(), null);
            Some(*Box::from_raw(ptr).downcast().unwrap())
        }
    }
}

impl<T> Drop for StateObject<T> {
    fn drop(&mut self) {
        unsafe {
            drop(ZObject::from_raw(self.as_mut_ptr()));
        }
    }
}

impl<T> Deref for StateObject<T> {
    type Target = StateObj<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl<T> DerefMut for StateObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap() }
    }
}

impl<T> Debug for StateObject<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "StateObject")
    }
}

fn common_fmt(this: &ZObj, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    let mut d = f.debug_struct(name);
    match this.get_class().get_name().to_c_str() {
        Ok(class_name) => {
            d.field("class", &class_name);
        }
        Err(e) => {
            d.field("class", &e);
        }
    }
    d.field("handle", &this.handle());
    d.finish()
}
