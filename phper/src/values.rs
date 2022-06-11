// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zval].

use crate::{
    alloc::EBox,
    arrays::{ZArr, ZArray},
    classes::ClassEntry,
    errors::{NotRefCountedTypeError, Throwable, TypeError},
    functions::{call_internal, ZendFunction},
    objects::{Object, StatelessObject},
    resources::Resource,
    strings::{ZStr, ZString},
    sys::*,
    types::TypeInfo,
    utils::ensure_end_with_zero,
};
use indexmap::map::IndexMap;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    marker::PhantomData,
    mem::{transmute, zeroed, MaybeUninit},
    ops::{Deref, DerefMut},
    os::raw::c_int,
    str,
    str::Utf8Error,
};

/// Wrapper of [crate::sys::zend_execute_data].
#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    /// # Safety
    ///
    /// Get value from union.
    #[inline]
    pub unsafe fn common_num_args(&self) -> u32 {
        (*self.inner.func).common.num_args
    }

    /// # Safety
    ///
    /// Get value from union.
    #[inline]
    pub unsafe fn common_required_num_args(&self) -> u16 {
        (*self.inner.func).common.required_num_args as u16
    }

    /// # Safety
    ///
    /// Get value from union.
    #[inline]
    pub unsafe fn common_arg_info(&self) -> *mut zend_arg_info {
        (*self.inner.func).common.arg_info
    }

    /// # Safety
    ///
    /// Get value from union.
    #[inline]
    pub unsafe fn num_args(&self) -> u16 {
        self.inner.This.u2.num_args as u16
    }

    /// # Safety
    ///
    /// From inner raw pointer.
    pub unsafe fn func(&self) -> &ZendFunction {
        ZendFunction::from_mut_ptr(self.inner.func)
    }

    /// # Safety
    ///
    /// The type of `T` should be careful.
    pub unsafe fn get_this<T>(&mut self) -> Option<&mut Object<T>> {
        let ptr = phper_get_this(&mut self.inner) as *mut ZVal;
        ptr.as_mut().map(|val| val.as_mut_object_unchecked())
    }

    /// TODO Do not return owned object, because usually Val should not be drop.
    pub(crate) unsafe fn get_parameters_array(&mut self) -> Vec<ZVal> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        if num_args > 0 {
            _zend_get_parameters_array_ex(num_args.into(), arguments.as_mut_ptr());
        }
        transmute(arguments)
    }

    pub fn get_parameter(&mut self, index: usize) -> &mut ZVal {
        unsafe {
            let val = phper_execute_data_call_arg(self.as_mut_ptr(), index as c_int);
            ZVal::from_mut_ptr(val)
        }
    }
}

/// Wrapper of [crate::sys::zval].
///
/// TODO Refactor `as_*`, to `to_*` or return reference.
#[repr(transparent)]
pub struct ZVal {
    inner: zval,
    _p: PhantomData<*mut ()>,
}

impl ZVal {
    pub unsafe fn from_ptr<'a>(ptr: *const zend_string) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_string) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    pub const fn as_ptr(&self) -> *const zval {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zval {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> zval {
        self.inner
    }

    pub fn get_type_info(&self) -> TypeInfo {
        let t = unsafe { self.inner.u1.type_info };
        t.into()
    }

    fn get_type_name(&self) -> crate::Result<String> {
        self.get_type_info().get_base_type_name()
    }

    fn set_type(&mut self, t: TypeInfo) {
        self.inner.u1.type_info = t.into_raw();
    }

    pub fn as_null(&self) -> crate::Result<()> {
        if self.get_type_info().is_null() {
            Ok(())
        } else {
            Err(self.must_be_type_error("null"))
        }
    }

    pub fn as_bool(&self) -> crate::Result<bool> {
        let t = self.get_type_info();
        if t.is_true() {
            Ok(true)
        } else if t.is_false() {
            Ok(false)
        } else {
            Err(self.must_be_type_error("bool"))
        }
    }

    pub fn as_long(&self) -> crate::Result<i64> {
        if self.get_type_info().is_long() {
            unsafe { Ok(self.inner.value.lval) }
        } else {
            Err(self.must_be_type_error("int"))
        }
    }

    pub fn as_long_value(&self) -> i64 {
        unsafe { phper_zval_get_long(&self.inner as *const _ as *mut _) }
    }

    pub fn as_double(&self) -> crate::Result<f64> {
        if self.get_type_info().is_double() {
            unsafe { Ok(self.inner.value.dval) }
        } else {
            Err(self.must_be_type_error("float"))
        }
    }

    pub fn to_str(&self) -> crate::Result<&str> {
        Ok(self.as_zend_string()?.to_str()?)
    }

    pub fn to_string(&self) -> crate::Result<String> {
        if self.get_type_info().is_string() {
            unsafe {
                let zs = ZStr::from_ptr(self.inner.value.str_);
                Ok(zs.to_str()?.to_owned())
            }
        } else {
            Err(self.must_be_type_error("string"))
        }
    }

    pub fn as_bytes(&self) -> crate::Result<&[u8]> {
        Ok(self.as_zend_string()?.as_ref())
    }

    pub fn as_zend_string(&self) -> crate::Result<&ZStr> {
        if self.get_type_info().is_string() {
            unsafe {
                let zs = ZStr::from_ptr(self.inner.value.str_);
                Ok(zs)
            }
        } else {
            Err(self.must_be_type_error("string"))
        }
    }

    pub fn as_string_value(&self) -> Result<String, Utf8Error> {
        unsafe {
            let s = phper_zval_get_string(&self.inner as *const _ as *mut _);
            ZStr::from_ptr(s).to_str().map(ToOwned::to_owned)
        }
    }

    pub fn as_array(&self) -> crate::Result<&ZArray> {
        if self.get_type_info().is_array() {
            unsafe {
                let ptr = self.inner.value.arr;
                Ok(ZArray::from_mut_ptr(ptr).unwrap())
            }
        } else {
            Err(self.must_be_type_error("array"))
        }
    }

    pub fn as_mut_array(&mut self) -> crate::Result<&mut ZArray> {
        if self.get_type_info().is_array() {
            unsafe {
                let ptr = self.inner.value.arr;
                Ok(ZArray::from_mut_ptr(ptr).unwrap())
            }
        } else {
            Err(self.must_be_type_error("array"))
        }
    }

    pub fn as_object(&self) -> crate::Result<&Object<()>> {
        if self.get_type_info().is_object() {
            unsafe {
                let ptr = self.inner.value.obj;
                Ok(Object::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("object"))
        }
    }

    pub fn as_mut_object(&mut self) -> crate::Result<&mut Object<()>> {
        if self.get_type_info().is_object() {
            unsafe {
                let ptr = self.inner.value.obj;
                Ok(Object::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("object"))
        }
    }

    pub(crate) unsafe fn as_mut_object_unchecked<T>(&mut self) -> &mut Object<T> {
        // TODO Fix the object type assertion.
        // assert!(dbg!(val.get_type()).is_object());

        let object = Object::from_mut_ptr(self.inner.value.obj);
        let class = object.get_class();
        ClassEntry::check_type_id(class).unwrap();
        object
    }

    pub fn as_resource(&self) -> crate::Result<&Resource> {
        if self.get_type_info().is_resource() {
            unsafe {
                let ptr = self.inner.value.res;
                Ok(Resource::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("resource"))
        }
    }

    pub fn as_mut_resource(&mut self) -> crate::Result<&mut Resource> {
        if self.get_type_info().is_resource() {
            unsafe {
                let ptr = self.inner.value.res;
                Ok(Resource::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("resource"))
        }
    }

    // TODO Error tip, not only for function arguments, should change.
    fn must_be_type_error(&self, expect_type: &str) -> crate::Error {
        match self.get_type_name() {
            Ok(type_name) => {
                let message = format!("must be of type {}, {} given", expect_type, type_name);
                TypeError::new(message).into()
            }
            Err(e) => e,
        }
    }

    /// Only add refcount.
    ///
    /// TODO Make a reference type to wrap self.
    pub fn duplicate(&mut self) -> Result<EBox<Self>, NotRefCountedTypeError> {
        unsafe {
            if !phper_z_refcounted_p(self.as_mut_ptr()) {
                Err(NotRefCountedTypeError)
            } else {
                (*self.inner.value.counted).gc.refcount += 1;
                let val = EBox::from_raw(self.as_mut_ptr().cast());
                Ok(val)
            }
        }
    }

    /// Call only when self is a callable.
    ///
    /// # Errors
    ///
    /// Return Err when self is not callable.
    pub fn call(&mut self, arguments: impl AsMut<[ZVal]>) -> crate::Result<EBox<ZVal>> {
        let none: Option<&mut StatelessObject> = None;
        call_internal(self, none, arguments)
    }
}

impl Clone for ZVal {
    fn clone(&self) -> Self {
        let mut val = ZVal::from(());
        unsafe {
            phper_zval_copy(val.as_mut_ptr(), self.as_ptr());
            if val.get_type().is_string() {
                phper_separate_string(val.as_mut_ptr());
            } else if val.get_type().is_array() {
                phper_separate_array(val.as_mut_ptr());
            }
        }
        val
    }
}

impl Drop for ZVal {
    fn drop(&mut self) {
        unsafe {
            phper_zval_ptr_dtor(self.as_mut_ptr());
        }
    }
}

impl From<()> for ZVal {
    fn from(_: ()) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_null(val.as_mut_ptr());
            val.assume_init()
        }
    }
}

impl From<bool> for ZVal {
    fn from(b: bool) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            if b {
                phper_zval_true(val.as_mut_ptr());
            } else {
                phper_zval_false(val.as_mut_ptr());
            }
            val.assume_init()
        }
    }
}

impl From<i32> for ZVal {
    fn from(i: i32) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_long(val.as_mut_ptr(), i.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<i64> for ZVal {
    fn from(i: i64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_long(val.as_mut_ptr(), i.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<f32> for ZVal {
    fn from(f: f32) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_double(val.as_mut_ptr(), f.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<f64> for ZVal {
    fn from(f: f64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_double(val.as_mut_ptr(), f.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<&str> for ZVal {
    fn from(s: &str) -> Self {
        ZVal::from(s.as_bytes())
    }
}

impl<const N: usize> From<&[u8; N]> for ZVal {
    fn from(b: &[u8; N]) -> Self {
        ZVal::from(&b[..])
    }
}

impl From<&[u8]> for ZVal {
    fn from(b: &[u8]) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_stringl(
                val.as_mut_ptr(),
                b.as_ptr().cast(),
                b.len().try_into().unwrap(),
            );
            val.assume_init()
        }
    }
}

impl From<&ZStr> for ZVal {
    fn from(s: &ZStr) -> Self {
        ZVal::from(s.to_bytes())
    }
}

impl From<ZString> for ZVal {
    fn from(s: ZString) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_str(val.as_mut_ptr(), s.into_raw());
            val.assume_init()
        }
    }
}

impl From<&ZArr> for ZVal {
    fn from(arr: &ZArr) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_arr(val.as_mut_ptr(), zend_array_dup(arr.as_mut_ptr()));
            val.assume_init()
        }
    }
}

impl From<ZArray> for ZVal {
    fn from(arr: ZArray) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_arr(val.as_mut_ptr(), arr.into_raw());
            val.assume_init()
        }
    }
}

// // TODO Support chain call for PHP object later, now only support return
// owned // object.
// impl<T> SetVal for EBox<Object<T>> {
//     unsafe fn set_val(self, val: &mut ZVal) {
//         let object = EBox::into_raw(self);
//         val.inner.value.obj = object.cast();
//         val.set_type(TypeInfo::object_ex());
//     }
// }

// impl<T: SetVal> SetVal for Option<T> {
//     unsafe fn set_val(self, val: &mut ZVal) {
//         match self {
//             Some(t) => SetVal::set_val(t, val),
//             None => SetVal::set_val((), val),
//         }
//     }
// }

// impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
//     unsafe fn set_val(self, val: &mut ZVal) {
//         match self {
//             Ok(t) => t.set_val(val),
//             Err(e) => {
//                 let class_entry = e.class_entry();
//                 let message = ensure_end_with_zero(e.message());
//                 zend_throw_exception(
//                     class_entry.as_ptr() as *mut _,
//                     message.as_ptr().cast(),
//                     e.code() as i64,
//                 );
//                 SetVal::set_val((), val);
//             }
//         }
//     }
// }

// impl SetVal for ZVal {
//     unsafe fn set_val(mut self, val: &mut ZVal) {
//         phper_zval_copy(val.as_mut_ptr(), self.as_mut_ptr());
//     }
// }

// impl SetVal for EBox<ZVal> {
//     unsafe fn set_val(self, val: &mut ZVal) {
//         phper_zval_zval(val.as_mut_ptr(), EBox::into_raw(self).cast(), 0, 1);
//     }
// }
