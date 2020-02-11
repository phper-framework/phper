#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null;

#[macro_use]
mod macros;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub const PHP_EXTENSION_BUILD: *const c_char = c_str_ptr!(env!("PHP_EXTENSION_BUILD"));

pub fn new_c_str_from_ptr_unchecked<'a>(ptr: *const c_char) -> &'a CStr {
    unsafe { CStr::from_ptr(ptr) }
}

impl Default for _zend_function_entry {
    fn default() -> Self {
        Self {
            fname: null(),
            handler: None,
            arg_info: null(),
            num_args: 0,
            flags: 0,
        }
    }
}

pub type InternalRawFunction =
    unsafe extern "C" fn(execute_data: *mut zend_execute_data, return_value: *mut zval);
