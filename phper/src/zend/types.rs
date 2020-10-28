use crate::sys::{zend_execute_data, zval};
use crate::php_function;

pub struct ZendExecuteData {
    raw: *mut zend_execute_data,
}

pub struct ZVal {
    raw: *mut zval,
}
