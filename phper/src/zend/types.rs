use crate::sys::{zend_execute_data, zval};
use crate::php_function;

pub struct ExecuteData {
    raw: *mut zend_execute_data,
}

impl ExecuteData {
    pub fn from_raw(execute_data: *mut zend_execute_data) -> Self {
        Self { raw: execute_data }
    }

    pub fn num_args(&self) -> usize {
        unsafe {
            (*self.raw).This.u2.num_args as usize
        }
    }
}

pub struct Val {
    inner: *mut zval,
}

impl Val {
    pub fn from_raw(val: *mut zval) -> Self {
        Self { inner: val }
    }
}
