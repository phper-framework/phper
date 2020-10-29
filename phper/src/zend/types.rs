use crate::sys::{zend_execute_data, zval};
use crate::php_function;

pub struct ExecuteData {
    raw: *mut zend_execute_data,
}

pub struct Val {
    raw: *mut zval,
}
