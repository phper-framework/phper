//! Apis relate to [crate::sys::zval].

use crate::{
    alloc::EBox,
    arrays::Array,
    errors::Throwable,
    functions::ZendFunction,
    objects::Object,
    sys::*,
    types::{get_type_by_const, Type},
    utils::ensure_end_with_zero,
    TypeError,
};
use indexmap::map::IndexMap;
use std::{
    collections::HashMap,
    mem::{transmute, zeroed},
    os::raw::c_char,
    slice::from_raw_parts,
    str,
    str::Utf8Error,
    sync::atomic::Ordering,
};

/// Wrapper of [crate::sys::zend_execute_data].
#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    #[inline]
    pub const fn new(inner: zend_execute_data) -> Self {
        Self { inner }
    }

    #[inline]
    pub unsafe fn from_mut<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    #[inline]
    pub unsafe fn common_num_args(&self) -> u32 {
        (*self.inner.func).common.num_args
    }

    #[inline]
    pub unsafe fn common_required_num_args(&self) -> u16 {
        (*self.inner.func).common.required_num_args as u16
    }

    #[inline]
    pub unsafe fn common_arg_info(&self) -> *mut zend_arg_info {
        (*self.inner.func).common.arg_info
    }

    #[inline]
    pub unsafe fn num_args(&self) -> u16 {
        self.inner.This.u2.num_args as u16
    }

    pub unsafe fn func(&self) -> &ZendFunction {
        ZendFunction::from_mut_ptr(self.inner.func)
    }

    #[inline]
    pub unsafe fn get_this(&mut self) -> *mut Val {
        phper_get_this(&mut self.inner).cast()
    }

    pub(crate) unsafe fn get_parameters_array(&mut self) -> Vec<Val> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        _zend_get_parameters_array_ex(num_args.into(), arguments.as_mut_ptr());
        transmute(arguments)
    }
}

/// Wrapper of [crate::sys::zval].
#[repr(transparent)]
pub struct Val {
    inner: zval,
}

impl Val {
    pub fn new<T: SetVal>(t: T) -> Self {
        let mut val = unsafe { zeroed::<Val>() };
        SetVal::set_val(t, &mut val);
        val
    }

    pub fn null() -> Self {
        Self::new(())
    }

    #[inline]
    pub const fn from_inner(inner: zval) -> Self {
        Self { inner }
    }

    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zval {
        &mut self.inner
    }

    pub fn get_type(&self) -> Type {
        let t = unsafe { self.inner.u1.type_info };
        t.into()
    }

    fn get_type_name(&self) -> crate::Result<String> {
        get_type_by_const(self.get_type() as u32)
    }

    fn set_type(&mut self, t: Type) {
        self.inner.u1.type_info = t as u32;
    }

    pub fn as_null(&self) -> crate::Result<()> {
        if self.get_type().is_null() {
            Ok(())
        } else {
            Err(self.must_be_type_error("null").into())
        }
    }

    pub fn as_bool(&self) -> crate::Result<bool> {
        let t = self.get_type();
        if t.is_true() {
            Ok(true)
        } else if t.is_false() {
            Ok(false)
        } else {
            Err(self.must_be_type_error("bool").into())
        }
    }

    pub fn as_long(&self) -> crate::Result<i64> {
        if self.get_type().is_long() {
            unsafe { Ok(self.inner.value.lval) }
        } else {
            Err(self.must_be_type_error("long").into())
        }
    }

    pub fn as_long_value(&self) -> i64 {
        unsafe { phper_zval_get_long(&self.inner as *const _ as *mut _) }
    }

    pub fn as_double(&self) -> crate::Result<f64> {
        if self.get_type().is_double() {
            unsafe { Ok(self.inner.value.dval) }
        } else {
            Err(self.must_be_type_error("float").into())
        }
    }

    pub fn as_string(&self) -> crate::Result<String> {
        if self.get_type().is_string() {
            unsafe {
                let s = self.inner.value.str;
                let buf = from_raw_parts(&(*s).val as *const c_char as *const u8, (*s).len);
                let string = str::from_utf8(buf)?.to_string();
                Ok(string)
            }
        } else {
            Err(self.must_be_type_error("string").into())
        }
    }

    pub fn as_string_value(&self) -> Result<String, Utf8Error> {
        unsafe {
            let s = phper_zval_get_string(&self.inner as *const _ as *mut _);
            let buf = from_raw_parts(&(*s).val as *const c_char as *const u8, (*s).len);
            let result = str::from_utf8(buf).map(ToString::to_string);
            phper_zend_string_release(s);
            result
        }
    }

    pub fn as_array(&self) -> crate::Result<&Array> {
        if self.get_type().is_array() {
            unsafe {
                let ptr = self.inner.value.arr;
                Ok(Array::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("array").into())
        }
    }

    pub fn as_object(&self) -> crate::Result<&Object> {
        if self.get_type().is_object() {
            unsafe {
                let ptr = self.inner.value.obj;
                Ok(Object::from_mut_ptr(ptr))
            }
        } else {
            Err(self.must_be_type_error("object").into())
        }
    }

    pub(crate) unsafe fn as_mut_object_unchecked(&self) -> &mut Object {
        Object::from_mut_ptr(self.inner.value.obj)
    }

    fn must_be_type_error(&self, expect_type: &str) -> crate::Error {
        match self.get_type_name() {
            Ok(type_name) => {
                let message = format!("must be of type {}, {} given", expect_type, type_name);
                TypeError::new(message).into()
            }
            Err(e) => e.into(),
        }
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        let t = self.get_type();
        unsafe {
            if t.is_string() {
                phper_zend_string_release(self.inner.value.str);
            } else if t.is_array() {
                zend_hash_destroy(self.inner.value.arr);
            } else if t.is_object() {
                zend_objects_destroy_object(self.inner.value.obj);
            }
        }
    }
}

/// The trait for setting the value of [Val], mainly as the return value of
/// [crate::functions::Function] and [crate::functions::Method], and initializer of [Val].
pub trait SetVal {
    fn set_val(self, val: &mut Val);
}

impl SetVal for () {
    fn set_val(self, val: &mut Val) {
        val.set_type(Type::Null);
    }
}

impl SetVal for bool {
    fn set_val(self, val: &mut Val) {
        val.set_type(if self { Type::True } else { Type::False });
    }
}

impl SetVal for i32 {
    fn set_val(self, val: &mut Val) {
        SetVal::set_val(self as i64, val)
    }
}

impl SetVal for u32 {
    fn set_val(self, val: &mut Val) {
        SetVal::set_val(self as i64, val)
    }
}

impl SetVal for i64 {
    fn set_val(self, val: &mut Val) {
        val.set_type(Type::Long);
        unsafe {
            (*val.as_mut_ptr()).value.lval = self;
        }
    }
}

impl SetVal for f64 {
    fn set_val(self, val: &mut Val) {
        val.set_type(Type::Double);
        unsafe {
            (*val.as_mut_ptr()).value.dval = self;
        }
    }
}

impl SetVal for &str {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut_ptr(), self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for String {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut_ptr(), self.as_ptr().cast(), self.len());
        }
    }
}

impl<T: SetVal> SetVal for Vec<T> {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_array_init(val.as_mut_ptr());
            for (k, v) in self.into_iter().enumerate() {
                let v = EBox::new(Val::new(v));
                phper_zend_hash_index_update(
                    (*val.as_mut_ptr()).value.arr,
                    k as u64,
                    EBox::into_raw(v).cast(),
                );
            }
        }
    }
}

/// Setting the val to an array, Because of the feature of [std::collections::HashMap], the item
/// order of array is not guarantee.
impl<K: AsRef<str>, V: SetVal> SetVal for HashMap<K, V> {
    fn set_val(self, val: &mut Val) {
        map_set_val(self, val);
    }
}

/// Setting the val to an array, which preserves item order.
impl<K: AsRef<str>, V: SetVal> SetVal for IndexMap<K, V> {
    fn set_val(self, val: &mut Val) {
        map_set_val(self, val);
    }
}

fn map_set_val<K, V, I>(iterator: I, val: &mut Val)
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: SetVal,
{
    unsafe {
        phper_array_init(val.as_mut_ptr());
        for (k, v) in iterator.into_iter() {
            let k = k.as_ref();
            let v = EBox::new(Val::new(v));
            phper_zend_hash_str_update(
                (*val.as_mut_ptr()).value.arr,
                k.as_ptr().cast(),
                k.len(),
                EBox::into_raw(v).cast(),
            );
        }
    }
}

impl SetVal for EBox<Array> {
    fn set_val(self, val: &mut Val) {
        unsafe {
            let arr = EBox::into_raw(self);
            phper_zval_arr(val.as_mut_ptr(), arr.cast());
        }
    }
}

impl SetVal for EBox<Object> {
    fn set_val(self, val: &mut Val) {
        let object = EBox::into_raw(self);
        val.inner.value.obj = object.cast();
        val.set_type(Type::ObjectEx);
    }
}

impl<T: SetVal> SetVal for Option<T> {
    fn set_val(self, val: &mut Val) {
        match self {
            Some(t) => SetVal::set_val(t, val),
            None => SetVal::set_val((), val),
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(e) => unsafe {
                let class_entry = e.class_entry();
                let message = ensure_end_with_zero(e.message());
                zend_throw_exception(
                    class_entry.as_ptr() as *mut _,
                    message.as_ptr().cast(),
                    e.code() as i64,
                );
                SetVal::set_val((), val);
            },
        }
    }
}

impl SetVal for Val {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            phper_zval_copy(val.as_mut_ptr(), self.as_mut_ptr());
        }
    }
}
