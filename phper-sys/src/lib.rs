#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));
