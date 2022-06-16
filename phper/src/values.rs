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
    errors::{ExpectTypeError, NotRefCountedTypeError, Throwable, TypeError},
    functions::{call_internal, ZendFunction},
    objects::{Object, StatelessObject},
    resources::ZRes,
    strings::{ZStr, ZString},
    sys::*,
    types::TypeInfo,
    utils::ensure_end_with_zero,
};
use indexmap::map::IndexMap;
use phper_alloc::RefClone;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    marker::PhantomData,
    mem::{forget, transmute, zeroed, MaybeUninit},
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
    pub unsafe fn from_ptr<'a>(ptr: *const zval) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zval) -> &'a mut Self {
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

    #[inline]
    pub fn into_raw(mut self) -> *mut zval {
        let ptr = self.as_mut_ptr();
        forget(self);
        ptr
    }

    pub fn get_type_info(&self) -> TypeInfo {
        let t = unsafe { phper_z_type_info_p(self.as_ptr()) };
        t.into()
    }

    fn get_type_name(&self) -> crate::Result<String> {
        self.get_type_info().get_base_type_name()
    }

    fn set_type(&mut self, t: TypeInfo) {
        self.inner.u1.type_info = t.into_raw();
    }

    pub fn as_null(&self) -> Option<()> {
        self.expect_null().ok()
    }

    pub fn expect_null(&self) -> crate::Result<()> {
        if self.get_type_info().is_null() {
            Ok(())
        } else {
            Err(ExpectTypeError::new(TypeInfo::NULL, self.get_type_info()).into())
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.expect_bool().ok()
    }

    pub fn expect_bool(&self) -> crate::Result<bool> {
        let t = self.get_type_info();
        if t.is_true() {
            Ok(true)
        } else if t.is_false() {
            Ok(false)
        } else {
            Err(ExpectTypeError::new(TypeInfo::BOOL, self.get_type_info()).into())
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        self.expect_long().ok()
    }

    pub fn expect_long(&self) -> crate::Result<i64> {
        if self.get_type_info().is_long() {
            unsafe { Ok(phper_z_lval_p(self.as_ptr())) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::LONG, self.get_type_info()).into())
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        self.expect_double().ok()
    }

    pub fn expect_double(&self) -> crate::Result<f64> {
        if self.get_type_info().is_double() {
            unsafe { Ok(phper_z_dval_p(self.as_ptr())) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::DOUBLE, self.get_type_info()).into())
        }
    }

    pub fn as_z_str(&self) -> Option<&ZStr> {
        self.expect_z_str().ok()
    }

    pub fn expect_z_str(&self) -> crate::Result<&ZStr> {
        self.inner_expect_z_str().map(|x| &*x)
    }

    pub fn as_mut_z_str(&mut self) -> Option<&mut ZStr> {
        self.expect_mut_z_str().ok()
    }

    pub fn expect_mut_z_str(&mut self) -> crate::Result<&mut ZStr> {
        self.inner_expect_z_str()
    }

    fn inner_expect_z_str(&self) -> crate::Result<&mut ZStr> {
        if self.get_type_info().is_string() {
            unsafe { Ok(ZStr::from_mut_ptr(phper_z_str_p(self.as_ptr()))) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::STRING, self.get_type_info()).into())
        }
    }

    pub fn as_z_arr(&self) -> Option<&ZArr> {
        self.expect_z_arr().ok()
    }

    pub fn expect_z_arr(&self) -> crate::Result<&ZArr> {
        self.inner_expect_z_arr().map(|x| &*x)
    }

    pub fn as_mut_z_arr(&mut self) -> Option<&mut ZArr> {
        self.expect_mut_z_arr().ok()
    }

    pub fn expect_mut_z_arr(&mut self) -> crate::Result<&mut ZArr> {
        self.inner_expect_z_arr()
    }

    fn inner_expect_z_arr(&self) -> crate::Result<&mut ZArr> {
        if self.get_type_info().is_array() {
            unsafe { Ok(ZArr::from_mut_ptr(phper_z_arr_p(self.as_ptr()))) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::ARRAY, self.get_type_info()).into())
        }
    }

    pub fn as_object(&self) -> Option<&Object<()>> {
        self.expect_object().ok()
    }

    pub fn expect_object(&self) -> crate::Result<&Object<()>> {
        self.inner_expect_object().map(|x| &*x)
    }

    pub fn as_mut_object(&mut self) -> Option<&mut Object<()>> {
        self.expect_mut_object().ok()
    }

    pub fn expect_mut_object(&mut self) -> crate::Result<&mut Object<()>> {
        self.inner_expect_object()
    }

    fn inner_expect_object(&self) -> crate::Result<&mut Object<()>> {
        if self.get_type_info().is_object() {
            unsafe {
                let ptr = self.inner.value.obj;
                Ok(Object::from_mut_ptr(ptr))
            }
        } else {
            Err(ExpectTypeError::new(TypeInfo::OBJECT, self.get_type_info()).into())
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

    pub fn as_z_res(&self) -> Option<&ZRes> {
        self.expect_z_res().ok()
    }

    pub fn expect_z_res(&self) -> crate::Result<&ZRes> {
        self.inner_expect_res().map(|x| &*x)
    }

    pub fn as_mut_res(&mut self) -> Option<&mut ZRes> {
        self.expect_mut_res().ok()
    }

    pub fn expect_mut_res(&mut self) -> crate::Result<&mut ZRes> {
        self.inner_expect_res()
    }

    fn inner_expect_res(&self) -> crate::Result<&mut ZRes> {
        if self.get_type_info().is_object() {
            unsafe { Ok(ZRes::from_mut_ptr(phper_z_res_p(self.as_ptr()))) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::RESOURCE, self.get_type_info()).into())
        }
    }

    pub fn convert_to_long(&mut self) {
        unsafe {
            phper_convert_to_long(self.as_mut_ptr());
        }
    }

    pub fn convert_to_string(&mut self) {
        unsafe {
            phper_convert_to_string(self.as_mut_ptr());
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

impl Default for ZVal {
    #[inline]
    fn default() -> Self {
        ZVal::from(())
    }
}

impl Clone for ZVal {
    fn clone(&self) -> Self {
        let mut val = ZVal::default();
        unsafe {
            phper_zval_copy(val.as_mut_ptr(), self.as_ptr());
            if val.get_type_info().is_string() {
                phper_separate_string(val.as_mut_ptr());
            } else if val.get_type_info().is_array() {
                phper_separate_array(val.as_mut_ptr());
            }
        }
        val
    }
}

impl RefClone for ZVal {
    fn ref_clone(&mut self) -> Self {
        let mut val = ZVal::default();
        unsafe {
            phper_zval_copy(val.as_mut_ptr(), self.as_ptr());
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
            phper_zval_null(val.as_mut_ptr().cast());
            val.assume_init()
        }
    }
}

impl From<bool> for ZVal {
    fn from(b: bool) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            if b {
                phper_zval_true(val.as_mut_ptr().cast());
            } else {
                phper_zval_false(val.as_mut_ptr().cast());
            }
            val.assume_init()
        }
    }
}

impl From<i64> for ZVal {
    fn from(i: i64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_long(val.as_mut_ptr().cast(), i.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<f64> for ZVal {
    fn from(f: f64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_double(val.as_mut_ptr().cast(), f.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<&[u8]> for ZVal {
    fn from(b: &[u8]) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_stringl(
                val.as_mut_ptr().cast(),
                b.as_ptr().cast(),
                b.len().try_into().unwrap(),
            );
            val.assume_init()
        }
    }
}

impl From<Vec<u8>> for ZVal {
    fn from(b: Vec<u8>) -> Self {
        ZVal::from(&b[..])
    }
}

impl From<&str> for ZVal {
    fn from(s: &str) -> Self {
        ZVal::from(s.as_bytes())
    }
}

impl From<String> for ZVal {
    fn from(s: String) -> Self {
        ZVal::from(s.as_bytes())
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
            phper_zval_str(val.as_mut_ptr().cast(), s.into_raw());
            val.assume_init()
        }
    }
}

impl From<&ZArr> for ZVal {
    fn from(arr: &ZArr) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_arr(
                val.as_mut_ptr().cast(),
                zend_array_dup(arr.as_ptr() as *mut _),
            );
            val.assume_init()
        }
    }
}

impl From<ZArray> for ZVal {
    fn from(arr: ZArray) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_arr(val.as_mut_ptr().cast(), arr.into_raw());
            val.assume_init()
        }
    }
}

// // TODO Support chain call for PHP object later, now only support return
// owned // object.
// impl<T> Into<ZVal> for EBox<Object<T>> {
//     unsafe fn set_val(self, val: &mut ZVal) {
//         let object = EBox::into_raw(self);
//         val.inner.value.obj = object.cast();
//         val.set_type(TypeInfo::object_ex());
//     }
// }

impl<T> From<Object<T>> for ZVal {
    fn from(mut o: Object<T>) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            todo!();
            val.assume_init()
        }
    }
}

impl<T: Into<ZVal>> From<Option<T>> for ZVal {
    fn from(o: Option<T>) -> Self {
        match o {
            Some(t) => t.into(),
            None => ().into(),
        }
    }
}

impl<T: Into<ZVal>> From<EBox<T>> for ZVal {
    fn from(t: EBox<T>) -> Self {
        t.into_inner().into()
    }
}

impl<T: Into<ZVal>, E: Throwable> From<Result<T, E>> for ZVal {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => t.into(),
            Err(e) => {
                let class_entry = e.class_entry();
                let message = ensure_end_with_zero(e.message());
                unsafe {
                    zend_throw_exception(
                        class_entry.as_ptr() as *mut _,
                        message.as_ptr().cast(),
                        e.code() as i64,
                    );
                }
                ZVal::from(())
            }
        }
    }
}
