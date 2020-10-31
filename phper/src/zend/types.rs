use crate::sys::{zend_execute_data, zval, zend_type, phper_zval_get_type, IS_STRING};
use std::ffi::CStr;

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

pub struct Val {
    raw: *mut zval,
}

impl Val {
    #[inline]
    pub fn from_raw(val: *mut zval) -> Self {
        Self { raw: val }
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
}
