#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;
use std::ffi::CStr;

#[macro_use]
mod macros;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub const PHP_BUILD_ID: *const c_char = c_str_ptr!(env!("PHP_BUILD_ID"));

pub fn new_c_str_from_ptr_unchecked<'a>(ptr: *const c_char) -> &'a CStr {
    unsafe { CStr::from_ptr(ptr) }
}
