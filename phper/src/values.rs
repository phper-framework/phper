use std::{
    borrow::Cow,
    cell::Cell,
    ffi::{c_void, CStr},
    mem::zeroed,
    os::raw::{c_char, c_int},
    ptr::null_mut,
    slice, str,
};

use crate::{
    c_str_ptr,
    classes::{This, Visibility},
    sys::{
        self, _zend_get_parameters_array_ex, phper_get_this, phper_init_class_entry_ex,
        phper_z_strval_p, phper_zend_string_release, phper_zval_get_long, phper_zval_get_string,
        phper_zval_get_type, phper_zval_stringl, phper_zval_zval, zend_arg_info, zend_class_entry,
        zend_declare_property_bool, zend_declare_property_long, zend_declare_property_null,
        zend_declare_property_stringl, zend_execute_data, zend_long, zend_parse_parameters,
        zend_read_property, zend_register_internal_class, zend_throw_exception,
        zend_update_property_bool, zend_update_property_long, zend_update_property_null,
        zend_update_property_stringl, zval, IS_DOUBLE, IS_FALSE, IS_LONG, IS_NULL, IS_TRUE,
        ZEND_RESULT_CODE_SUCCESS,
    },
    throws::Throwable,
};
use std::slice::from_raw_parts;

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

    pub fn as_mut(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    #[inline]
    pub unsafe fn common_num_args(&self) -> u32 {
        (*self.inner.func).common.num_args
    }

    #[inline]
    pub unsafe fn common_required_num_args(&self) -> u32 {
        (*self.inner.func).common.required_num_args
    }

    #[inline]
    pub unsafe fn common_arg_info(&self) -> *mut zend_arg_info {
        (*self.inner.func).common.arg_info
    }

    #[inline]
    pub unsafe fn num_args(&self) -> u32 {
        self.inner.This.u2.num_args
    }

    #[inline]
    pub unsafe fn get_this(&mut self) -> &mut This {
        This::from_mut(phper_get_this(&mut self.inner))
    }

    pub unsafe fn get_parameters_array(&mut self) -> Vec<Val> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        _zend_get_parameters_array_ex(num_args as c_int, arguments.as_mut_ptr());
        arguments.into_iter().map(Val::new).collect()
    }
}

#[repr(transparent)]
pub struct Val {
    inner: zval,
}

impl Val {
    #[inline]
    pub const fn new(inner: zval) -> Self {
        Self { inner }
    }

    #[inline]
    pub unsafe fn from_mut<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }

    pub fn as_mut(&mut self) -> *mut zval {
        &mut self.inner
    }

    unsafe fn type_info(&mut self) -> &mut u32 {
        &mut self.inner.u1.type_info
    }

    pub fn as_string(&mut self) -> String {
        unsafe {
            let s = phper_zval_get_string(&mut self.inner);
            let buf = from_raw_parts(&(*s).val as *const i8 as *const u8, (*s).len);
            phper_zend_string_release(s);
            str::from_utf8(buf).unwrap().to_string()
        }
    }

    pub fn as_i64(&mut self) -> i64 {
        unsafe { phper_zval_get_long(&mut self.inner) }
    }
}

pub trait SetVal {
    fn set_val(self, val: &mut Val);
}

impl SetVal for () {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = IS_NULL;
        }
    }
}

impl SetVal for bool {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = if self { IS_TRUE } else { IS_FALSE };
        }
    }
}

impl SetVal for i32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for u32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for i64 {
    fn set_val(self, val: &mut Val) {
        unsafe {
            (*val.as_mut()).value.lval = self;
            (*val.as_mut()).u1.type_info = IS_LONG;
        }
    }
}

impl SetVal for f64 {
    fn set_val(self, val: &mut Val) {
        unsafe {
            (*val.as_mut()).value.dval = self;
            (*val.as_mut()).u1.type_info = IS_DOUBLE;
        }
    }
}

impl SetVal for &str {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for String {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl<T: SetVal> SetVal for Option<T> {
    fn set_val(self, val: &mut Val) {
        match self {
            Some(t) => t.set_val(val),
            None => ().set_val(val),
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(_e) => unsafe {
                zend_throw_exception(null_mut(), c_str_ptr!(""), 0);
                todo!();
            },
        }
    }
}

impl SetVal for Val {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            phper_zval_zval(val.as_mut(), self.as_mut(), 1, 0);
        }
    }
}
