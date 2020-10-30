use crate::sys::{zend_execute_data, zval};
use crate::Result as PHPerResult;

pub type ValueResult = PHPerResult<Option<Value>>;

pub type FunctionType<'a> = fn(Parameters) -> ValueResult;

pub struct Parameters {
    pub(crate) execute_data: *mut zend_execute_data,
}

impl Parameters {
    #[inline]
    pub fn num_args(&self) -> usize {
        unsafe { (*self.execute_data).This.u2.num_args as usize }
    }

    #[inline]
    pub fn execute_data(self) -> *mut zend_execute_data {
        self.execute_data
    }
}

pub struct Value {
    pub(crate) ptr: *mut zval,
}
