// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zval].

use crate::{
    alloc::EBox,
    arrays::{ZArr, ZArray},
    errors::ExpectTypeError,
    functions::{ZFunc, call_internal},
    objects::{StateObject, ZObj, ZObject},
    references::ZRef,
    resources::ZRes,
    strings::{ZStr, ZString},
    sys::*,
    types::TypeInfo,
};
use phper_alloc::RefClone;
use std::{
    ffi::CStr,
    fmt,
    fmt::Debug,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit, transmute, zeroed},
    str,
};

/// Wrapper of [zend_execute_data].
#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
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
    #[allow(dead_code)]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_execute_data) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_execute_data) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_execute_data) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_execute_data {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    #[allow(dead_code)]
    pub fn as_mut_ptr(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    /// Gets common arguments count.
    #[inline]
    pub fn common_num_args(&self) -> u32 {
        unsafe { (*self.inner.func).common.num_args }
    }

    /// Gets common required arguments count.
    #[inline]
    pub fn common_required_num_args(&self) -> usize {
        unsafe { (*self.inner.func).common.required_num_args as usize }
    }

    /// Gets first common argument info.
    #[inline]
    pub fn common_arg_info(&self) -> *mut zend_arg_info {
        unsafe { (*self.inner.func).common.arg_info }
    }

    /// Gets arguments count.
    #[inline]
    pub fn num_args(&self) -> usize {
        unsafe { phper_zend_num_args(self.as_ptr()).try_into().unwrap() }
    }

    /// Gets associated function.
    pub fn func(&self) -> &ZFunc {
        unsafe { ZFunc::from_mut_ptr(self.inner.func) }
    }

    /// Gets associated `$this` object if exists.
    pub fn get_this(&mut self) -> Option<&ZObj> {
        unsafe {
            let val = ZVal::from_ptr(phper_get_this(&mut self.inner));
            val.as_z_obj()
        }
    }

    /// Gets associated mutable `$this` object if exists.
    pub fn get_this_mut(&mut self) -> Option<&mut ZObj> {
        unsafe {
            let val = ZVal::from_mut_ptr(phper_get_this(&mut self.inner));
            val.as_mut_z_obj()
        }
    }

    pub(crate) unsafe fn get_parameters_array(&mut self) -> Vec<ManuallyDrop<ZVal>> {
        unsafe {
            let num_args = self.num_args();
            let mut arguments = vec![zeroed::<zval>(); num_args];
            if num_args > 0 {
                phper_zend_get_parameters_array_ex(
                    num_args.try_into().unwrap(),
                    arguments.as_mut_ptr(),
                );
            }
            transmute(arguments)
        }
    }

    /// Gets parameter by index.
    pub fn get_parameter(&self, index: usize) -> &ZVal {
        unsafe {
            let val = phper_zend_call_var_num(self.as_ptr() as *mut _, index.try_into().unwrap());
            ZVal::from_ptr(val)
        }
    }

    /// Gets mutable parameter by index.
    pub fn get_mut_parameter(&mut self, index: usize) -> &mut ZVal {
        unsafe {
            let val = phper_zend_call_var_num(self.as_mut_ptr(), index.try_into().unwrap());
            ZVal::from_mut_ptr(val)
        }
    }
}

/// Wrapper of [zval].
#[repr(transparent)]
pub struct ZVal {
    inner: zval,
    _p: PhantomData<*mut ()>,
}

impl ZVal {
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
    pub unsafe fn from_ptr<'a>(ptr: *const zval) -> &'a Self {
        unsafe { (ptr as *const Self).as_ref().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zval) -> Option<&'a Self> {
        unsafe { (ptr as *const Self).as_ref() }
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zval) -> &'a mut Self {
        unsafe { (ptr as *mut Self).as_mut().expect("ptr should't be null") }
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zval) -> Option<&'a mut Self> {
        unsafe { (ptr as *mut Self).as_mut() }
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zval {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zval {
        &mut self.inner
    }

    /// Consumes the `ZVal`, returning the wrapped `zval`.
    #[inline]
    pub fn into_inner(self) -> zval {
        self.inner
    }

    /// Gets the type info of `ZVal`.
    pub fn get_type_info(&self) -> TypeInfo {
        let t = unsafe { phper_z_type_info_p(self.as_ptr()) };
        t.into()
    }

    /// Converts to null if `ZVal` is null.
    pub fn as_null(&self) -> Option<()> {
        self.expect_null().ok()
    }

    /// Converts to null if `ZVal` is null, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_null(&self) -> crate::Result<()> {
        if self.get_type_info().is_null() {
            Ok(())
        } else {
            Err(ExpectTypeError::new(TypeInfo::NULL, self.get_type_info()).into())
        }
    }

    /// Converts to bool if `ZVal` is bool.
    pub fn as_bool(&self) -> Option<bool> {
        self.expect_bool().ok()
    }

    /// Converts to bool if `ZVal` is bool, otherwise returns
    /// [`ExpectTypeError`].
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

    /// Converts to long if `ZVal` is long.
    pub fn as_long(&self) -> Option<i64> {
        self.expect_long().ok()
    }

    /// Converts to long if `ZVal` is long, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_long(&self) -> crate::Result<i64> {
        self.inner_expect_long().cloned()
    }

    /// Converts to mutable long if `ZVal` is long.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::values::ZVal;
    ///
    /// let mut val = ZVal::from(100);
    /// *val.as_mut_long().unwrap() += 100;
    /// assert_eq!(val.as_long().unwrap(), 200);
    /// ```
    pub fn as_mut_long(&mut self) -> Option<&mut i64> {
        self.expect_mut_long().ok()
    }

    /// Converts to mutable long if `ZVal` is long, otherwise returns
    /// [`ExpectTypeError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::values::ZVal;
    ///
    /// let mut val = ZVal::from(100);
    /// *val.expect_mut_long().unwrap() += 100;
    /// assert_eq!(val.expect_long().unwrap(), 200);
    /// ```
    pub fn expect_mut_long(&mut self) -> crate::Result<&mut i64> {
        self.inner_expect_long()
    }

    fn inner_expect_long(&self) -> crate::Result<&mut i64> {
        if self.get_type_info().is_long() {
            unsafe { Ok(phper_z_lval_p(self.as_ptr() as *mut _).as_mut().unwrap()) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::LONG, self.get_type_info()).into())
        }
    }

    /// Converts to double if `ZVal` is double.
    pub fn as_double(&self) -> Option<f64> {
        self.expect_double().ok()
    }

    /// Converts to double if `ZVal` is double, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_double(&self) -> crate::Result<f64> {
        self.inner_expect_double().cloned()
    }

    /// Converts to mutable double if `ZVal` is double.
    pub fn as_mut_double(&mut self) -> Option<&mut f64> {
        self.expect_mut_double().ok()
    }

    /// Converts to mutable double if `ZVal` is double, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_mut_double(&mut self) -> crate::Result<&mut f64> {
        self.inner_expect_double()
    }

    fn inner_expect_double(&self) -> crate::Result<&mut f64> {
        if self.get_type_info().is_double() {
            unsafe { Ok(phper_z_dval_p(self.as_ptr() as *mut _).as_mut().unwrap()) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::DOUBLE, self.get_type_info()).into())
        }
    }

    /// Converts to string if `ZVal` is string.
    pub fn as_z_str(&self) -> Option<&ZStr> {
        self.expect_z_str().ok()
    }

    /// Converts to string if `ZVal` is string, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_z_str(&self) -> crate::Result<&ZStr> {
        self.inner_expect_z_str().map(|x| &*x)
    }

    /// Converts to mutable string if `ZVal` is string.
    pub fn as_mut_z_str(&mut self) -> Option<&mut ZStr> {
        self.expect_mut_z_str().ok()
    }

    /// Converts to mutable string if `ZVal` is string, otherwise returns
    /// [`ExpectTypeError`].
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

    /// Converts to array if `ZVal` is array.
    pub fn as_z_arr(&self) -> Option<&ZArr> {
        self.expect_z_arr().ok()
    }

    /// Converts to array if `ZVal` is array, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_z_arr(&self) -> crate::Result<&ZArr> {
        self.inner_expect_z_arr().map(|x| &*x)
    }

    /// Converts to mutable array if `ZVal` is array.
    pub fn as_mut_z_arr(&mut self) -> Option<&mut ZArr> {
        self.expect_mut_z_arr().ok()
    }

    /// Converts to mutable array if `ZVal` is array, otherwise returns
    /// [`ExpectTypeError`].
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

    /// Converts to object if `ZVal` is object.
    pub fn as_z_obj(&self) -> Option<&ZObj> {
        self.expect_z_obj().ok()
    }

    /// Converts to object if `ZVal` is object, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_z_obj(&self) -> crate::Result<&ZObj> {
        self.inner_expect_z_obj().map(|x| &*x)
    }

    /// Converts to mutable object if `ZVal` is object.
    pub fn as_mut_z_obj(&mut self) -> Option<&mut ZObj> {
        self.expect_mut_z_obj().ok()
    }

    /// Converts to mutable object if `ZVal` is object, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_mut_z_obj(&mut self) -> crate::Result<&mut ZObj> {
        self.inner_expect_z_obj()
    }

    fn inner_expect_z_obj(&self) -> crate::Result<&mut ZObj> {
        if self.get_type_info().is_object() {
            unsafe {
                let ptr = phper_z_obj_p(self.as_ptr());
                Ok(ZObj::from_mut_ptr(ptr))
            }
        } else {
            Err(ExpectTypeError::new(TypeInfo::OBJECT, self.get_type_info()).into())
        }
    }

    /// Converts to resource if `ZVal` is resource.
    pub fn as_z_res(&self) -> Option<&ZRes> {
        self.expect_z_res().ok()
    }

    /// Converts to resource if `ZVal` is resource, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_z_res(&self) -> crate::Result<&ZRes> {
        self.inner_expect_z_res().map(|x| &*x)
    }

    /// Converts to mutable resource if `ZVal` is null.
    pub fn as_mut_z_res(&mut self) -> Option<&mut ZRes> {
        self.expect_mut_z_res().ok()
    }

    /// Converts to mutable resource if `ZVal` is resource, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_mut_z_res(&mut self) -> crate::Result<&mut ZRes> {
        self.inner_expect_z_res()
    }

    fn inner_expect_z_res(&self) -> crate::Result<&mut ZRes> {
        if self.get_type_info().is_resource() {
            unsafe { Ok(ZRes::from_mut_ptr(phper_z_res_p(self.as_ptr()))) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::RESOURCE, self.get_type_info()).into())
        }
    }

    /// Converts to reference if `ZVal` is reference.
    pub fn as_z_ref(&self) -> Option<&ZRef> {
        self.expect_z_ref().ok()
    }

    /// Converts to reference if `ZVal` is reference, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_z_ref(&self) -> crate::Result<&ZRef> {
        self.inner_expect_z_ref().map(|x| &*x)
    }

    /// Converts to mutable reference if `ZVal` is reference.
    pub fn as_mut_z_ref(&mut self) -> Option<&mut ZRef> {
        self.expect_mut_z_ref().ok()
    }

    /// Converts to mutable reference if `ZVal` is reference, otherwise returns
    /// [`ExpectTypeError`].
    pub fn expect_mut_z_ref(&mut self) -> crate::Result<&mut ZRef> {
        self.inner_expect_z_ref()
    }

    fn inner_expect_z_ref(&self) -> crate::Result<&mut ZRef> {
        if self.get_type_info().is_reference() {
            unsafe { Ok(ZRef::from_mut_ptr(phper_z_ref_p(self.as_ptr()))) }
        } else {
            Err(ExpectTypeError::new(TypeInfo::REFERENCE, self.get_type_info()).into())
        }
    }

    /// Internally convert to long.
    ///
    /// TODO To fix assertion failed.
    pub fn convert_to_long(&mut self) {
        unsafe {
            phper_convert_to_long(self.as_mut_ptr());
        }
    }

    /// Internally convert to string.
    ///
    /// TODO To fix assertion failed.
    pub fn convert_to_string(&mut self) {
        unsafe {
            phper_convert_to_string(self.as_mut_ptr());
        }
    }

    /// Call only when self is a callable (string or array or closure).
    ///
    /// # Errors
    ///
    /// Return Err when self is not callable, or called failed.
    #[inline]
    pub fn call(&mut self, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZVal> {
        call_internal(self, None, arguments)
    }
}

impl Debug for ZVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        struct null;

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        struct unknown;

        let mut d = f.debug_tuple("ZVal");

        let t = self.get_type_info();

        if t.is_null() {
            d.field(&null);
        } else if t.is_bool() {
            if let Some(v) = self.as_bool() {
                d.field(&v);
            }
        } else if t.is_long() {
            if let Some(v) = self.as_long() {
                d.field(&v);
            }
        } else if t.is_double() {
            if let Some(v) = self.as_double() {
                d.field(&v);
            }
        } else if t.is_string() {
            if let Some(v) = self.as_z_str() {
                d.field(&v);
            }
        } else if t.is_array() {
            if let Some(v) = self.as_z_arr() {
                d.field(&v);
            }
        } else if t.is_object() {
            if let Some(v) = self.as_z_obj() {
                d.field(&v);
            }
        } else if t.is_resource() {
            if let Some(v) = self.as_z_res() {
                d.field(&v);
            }
        } else if t.is_reference() {
            if let Some(v) = self.as_z_ref() {
                d.field(&v);
            }
        } else {
            d.field(&unknown);
        }

        d.finish()
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
    #[allow(clippy::useless_conversion)]
    fn from(i: i64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_long(val.as_mut_ptr().cast(), i.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<f64> for ZVal {
    #[allow(clippy::useless_conversion)]
    fn from(f: f64) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_double(val.as_mut_ptr().cast(), f.try_into().unwrap());
            val.assume_init()
        }
    }
}

impl From<&[u8]> for ZVal {
    #[allow(clippy::useless_conversion)]
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

impl From<&CStr> for ZVal {
    fn from(s: &CStr) -> Self {
        ZVal::from(s.to_bytes())
    }
}

impl From<String> for ZVal {
    fn from(s: String) -> Self {
        ZVal::from(s.as_bytes())
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

impl From<ZArray> for ZVal {
    fn from(arr: ZArray) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_arr(val.as_mut_ptr().cast(), arr.into_raw());
            val.assume_init()
        }
    }
}

impl From<ZObject> for ZVal {
    fn from(obj: ZObject) -> Self {
        unsafe {
            let mut val = MaybeUninit::<ZVal>::uninit();
            phper_zval_obj(val.as_mut_ptr().cast(), obj.into_raw());
            val.assume_init()
        }
    }
}

impl<T> From<StateObject<T>> for ZVal {
    fn from(obj: StateObject<T>) -> Self {
        ZVal::from(obj.into_z_object())
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
