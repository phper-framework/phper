#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

impl Default for _zend_function_entry {
    fn default() -> Self {
        Self {
            fname: 0 as *const c_char,
            handler: None,
            arg_info: 0 as *const _zend_internal_arg_info,
            num_args: 0,
            flags: 0,
        }
    }
}

#[repr(C)]
pub struct zend_function_entry_wrapper(pub *const zend_function_entry);

unsafe impl Sync for zend_function_entry_wrapper {}

#[repr(C)]
pub struct zend_module_entry_wrapper(pub *const zend_module_entry);

unsafe impl Sync for zend_module_entry_wrapper {}
