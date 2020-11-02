use crate::sys::{zend_execute_data, zval, zend_type, phper_zval_get_type, IS_STRING, IS_NULL,
                 IS_TRUE, IS_FALSE, phper_zval_stringl, zend_throw_exception,
                 zend_class_entry, phper_init_class_entry, zend_register_internal_class,
                 zend_declare_property_stringl, zend_declare_property, IS_LONG,
};
use crate::c_str_ptr;
use std::ffi::CStr;
use std::borrow::Cow;
use crate::zend::exceptions::Throwable;
use std::ptr::{null, null_mut};
use std::cell::Cell;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use crate::zend::api::FunctionEntries;
use std::mem::MaybeUninit;

pub struct ClassEntry {
    inner: Cell<*mut zend_class_entry>,
}

impl ClassEntry {
    pub const fn new() -> Self {
        Self {
            inner: Cell::new(null_mut()),
        }
    }

    pub const fn as_ptr(&self) -> *mut *mut zend_class_entry {
        self.inner.as_ptr()
    }

    pub fn get(&self) -> *mut zend_class_entry {
        self.inner.get()
    }

    pub fn init<const N: usize>(&self, class_name: *const c_char, functions: &FunctionEntries<N>) {
        unsafe {
            let mut class_ce = phper_init_class_entry(class_name, functions.as_ptr());
            *self.as_ptr() = zend_register_internal_class(&mut class_ce);
        }
    }

    pub fn declare_property(&self, name: impl AsRef<str>, value: impl SetVal, access_type: u32) {
        unsafe {
            let name = name.as_ref();
            let mut property: MaybeUninit<zval> = MaybeUninit::uninit();
            let mut property = Val::from_raw(property.as_mut_ptr());
            value.set_val(&mut property);
            zend_declare_property(self.get(), name.as_ptr().cast(), name.len(), property.as_ptr(), access_type as c_int);
        }
    }
}

unsafe impl Sync for ClassEntry {}

pub struct ExecuteData {
    raw: *mut zend_execute_data,
}

impl ExecuteData {
    pub fn from_raw(execute_data: *mut zend_execute_data) -> Self {
        Self { raw: execute_data }
    }

    #[inline]
    pub fn num_args(&self) -> usize {
        unsafe {
            (*self.raw).This.u2.num_args as usize
        }
    }

    #[inline]
    pub fn get_this(&self) -> &mut zval {
        unsafe {
            &mut (*self.raw).This
        }
    }

    #[inline]
    pub fn get_type(&self) -> zend_type {
        unsafe {
            phper_zval_get_type(self.get_this()).into()
        }
    }

    pub fn parse_parameters(&self) {

    }
}

#[repr(u32)]
pub enum ValType {
    UNDEF	 =      crate::sys::IS_UNDEF,
    NULL	=	    crate::sys::IS_NULL,
    FALSE	=       crate::sys::IS_FALSE,
    TRUE	=	    crate::sys::IS_TRUE,
    LONG	=	    crate::sys::IS_LONG,
    DOUBLE	=       crate::sys::IS_DOUBLE,
    STRING	=       crate::sys::IS_STRING,
    ARRAY	=       crate::sys::IS_ARRAY,
    OBJECT	=       crate::sys::IS_OBJECT,
    RESOURCE =	    crate::sys::IS_RESOURCE,
    REFERENCE =     crate::sys::IS_REFERENCE,
}

pub struct Val {
    raw: *mut zval,
}

impl Val {
    pub const fn from_raw(val: *mut zval) -> Self {
        Self {
            raw: val,
        }
    }

    pub const fn as_ptr(&self) -> *mut zval {
        self.raw
    }

    pub fn as_c_str(&self) -> Option<&CStr> {
        unsafe {
            if phper_zval_get_type(self.raw) as zend_type == IS_STRING  as zend_type {
                Some(CStr::from_ptr((&((*(*self.raw).value.str).val)).as_ptr().cast()))
            } else {
                None
            }
        }
    }

    unsafe fn type_info(&mut self) -> &mut u32 {
        &mut (*self.raw).u1.type_info
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
            *val.type_info() = if self {
                IS_TRUE
            } else {
                IS_FALSE
            };
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
            (*val.as_ptr()).value.lval = self;
            (*val.as_ptr()).u1.type_info = IS_LONG;
        }
    }
}

impl SetVal for &str {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.raw, self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for String {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.raw, self.as_ptr().cast(), self.len());
        }
    }
}

impl<T: SetVal> SetVal for Option<T> {
    fn set_val(self, val: &mut Val) {
        unsafe {
            match self {
                Some(t) => t.set_val(val),
                None => ().set_val(val),
            }
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(e) => {
                unsafe {
                    zend_throw_exception(null_mut(), c_str_ptr!("Fuck"), 0);
                }
            }
        }
    }
}

pub enum Value<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    String(String),
}

impl SetVal for Value<'_> {
    fn set_val(self, val: &mut Val) {
        match self {
            Value::Null => ().set_val(val),
            Value::Bool(b) => b.set_val(val),
            Value::Str(s) => s.set_val(val),
            Value::String(s) => s.set_val(val),
        }
    }
}