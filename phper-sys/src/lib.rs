#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;

#[macro_use]
mod macros;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub const PHP_BUILD_ID: *const c_char = c_str_ptr!(env!("PHP_BUILD_ID"));
