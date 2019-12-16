#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
mod macros;
mod zend;

include!(concat!(env!("OUT_DIR"), "/php_bindings.rs"));

pub use crate::zend::*;

#[repr(C)]
pub struct zend_function_entry_wrapper(pub *const zend_function_entry);

unsafe impl Sync for zend_function_entry_wrapper {}

#[repr(C)]
pub struct zend_module_entry_wrapper(pub *const zend_module_entry);

unsafe impl Sync for zend_module_entry_wrapper {}
