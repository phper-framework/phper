use crate::{
    arrays::Array,
    errors::Throwable,
    sys::*,
    types::{get_type_by_const, Type},
    utils::ensure_end_with_zero,
    TypeError,
};
use std::{
    mem::zeroed,
    os::raw::{c_char, c_int},
    slice::from_raw_parts,
    str,
    str::Utf8Error,
    sync::atomic::Ordering,
};

#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    #[inline]
    pub const fn new(inner: zend_execute_data) -> Self {
        Self { inner }
    }

    pub unsafe fn from_mut<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

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

    #[inline]
    pub unsafe fn get_this(&mut self) -> *mut Val {
        phper_get_this(&mut self.inner).cast()
    }

    pub(crate) unsafe fn get_parameters_array(&mut self) -> Vec<Val> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        _zend_get_parameters_array_ex(num_args.into(), arguments.as_mut_ptr());
        arguments.into_iter().map(Val::from_inner).collect()
    }
}

#[repr(transparent)]
pub struct Val {
    pub(crate) inner: zval,
}

impl Val {
    pub fn new<T: SetVal>(t: T) -> Self {
        let mut val = Self::empty();
        val.set(t);
        val
    }

    #[inline]
    pub const fn from_inner(inner: zval) -> Self {
        Self { inner }
    }

    pub unsafe fn from_mut<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }

    #[inline]
    pub(crate) fn empty() -> Self {
        Self {
            inner: unsafe { zeroed::<zval>() },
        }
    }

    pub fn null() -> Self {
        let mut val = Self::empty();
        val.set(());
        val
    }

    pub fn from_bool(b: bool) -> Self {
        let mut val = Self::empty();
        val.set(b);
        val
    }

    pub fn from_val(other: Val) -> Self {
        let mut val = Self::empty();
        val.set(other);
        val
    }

    pub fn as_mut_ptr(&mut self) -> *mut zval {
        &mut self.inner
    }

    pub fn set(&mut self, mut v: impl SetVal) {
        v.set_val(self);
    }

    #[inline]
    fn get_type(&self) -> Type {
        let t = unsafe { self.inner.u1.type_info };
        t.into()
    }

    fn get_type_name(&self) -> crate::Result<String> {
        get_type_by_const(unsafe { self.get_type() as u32 })
    }

    fn set_type(&mut self, t: Type) {
        unsafe {
            self.inner.u1.type_info = t as u32;
        }
    }

    pub fn as_bool(&self) -> crate::Result<bool> {
        match self.get_type() {
            Type::True => Ok(true),
            Type::False => Ok(false),
            _ => Err(self.must_be_type_error("bool").into()),
        }
    }

    pub fn as_i64(&self) -> crate::Result<i64> {
        if self.get_type() == Type::Long {
            unsafe { Ok(self.inner.value.lval) }
        } else {
            Err(self.must_be_type_error("long").into())
        }
    }

    pub fn as_i64_value(&self) -> i64 {
        unsafe { phper_zval_get_long(&self.inner as *const _ as *mut _) }
    }

    pub fn as_f64(&self) -> crate::Result<f64> {
        if self.get_type() == Type::Double {
            unsafe { Ok(self.inner.value.dval) }
        } else {
            Err(self.must_be_type_error("float").into())
        }
    }

    pub fn as_string(&self) -> crate::Result<String> {
        if self.get_type() == Type::String {
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
            let string = str::from_utf8(buf)?.to_string();
            phper_zend_string_release(s);
            Ok(string)
        }
    }

    pub fn as_array(&self) {}

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

impl SetVal for Array {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            let mut new_val = Val::empty();
            phper_zval_arr(new_val.as_mut_ptr(), self.as_mut_ptr());
            phper_zval_zval(
                val.as_mut_ptr(),
                new_val.as_mut_ptr(),
                true.into(),
                false.into(),
            );
        }
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
                let class = e
                    .class_entity()
                    .as_ref()
                    .expect("class entry is null pointer");
                let message = ensure_end_with_zero(&e);
                zend_throw_exception(
                    class.entry.load(Ordering::SeqCst).cast(),
                    message.as_ptr().cast(),
                    e.code() as i64,
                );
            },
        }
    }
}

impl SetVal for Val {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            phper_zval_copy_value(val.as_mut_ptr(), self.as_mut_ptr());
        }
    }
}
