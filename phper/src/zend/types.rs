use sys::zend_execute_data;
use sys::zval;

pub struct ZendExecuteData {
     ptr: *mut zend_execute_data,
}

impl ZendExecuteData {
     pub fn num_args(&self) -> usize {
         unsafe {
             (*self.ptr).This.u2.num_args as usize
         }
     }
}

pub struct ZVal {
     ptr: *mut zval,
}