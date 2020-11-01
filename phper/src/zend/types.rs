use crate::sys::{zend_execute_data, zval, zend_type, phper_zval_get_type, IS_STRING, IS_NULL, IS_TRUE, IS_FALSE, phper_zval_stringl};
use std::ffi::CStr;
use std::borrow::Cow;

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
    #[inline]
    pub fn from_raw(val: *mut zval) -> Self {
        Self {
            raw: val,
        }
    }

    pub fn get(&self) -> *mut zval {
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