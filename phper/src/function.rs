use crate::sys::{zend_execute_data, zval};
use std::ffi::CStr;
use std::os::raw::c_uchar;

#[derive(Debug)]
pub struct Function<'a> {
    pub(crate) name: &'a CStr,
    pub(crate) func: extern "C" fn(*mut zend_execute_data, *mut zval),
    pub(crate) arg_info: Option<ArgInfo<'a>>,
    pub(crate) flags: u32,
}

impl<'a> Function<'a> {
    pub fn new(name: &'a CStr, func: extern "C" fn(*mut zend_execute_data, *mut zval)) -> Self {
        Self {
            name,
            func,
            arg_info: None,
            flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct BeginArgInfo<'a> {
    name: &'a CStr,
    pass_by_ref: c_uchar,
    type_hint: Option<ArgType>,
    classname: Option<&'a CStr>,
    allow_null: bool,
}

impl<'a> BeginArgInfo<'a> {
    pub fn new(name: &'a CStr) -> Self {
        Self {
            name,
            pass_by_ref: 0,
            type_hint: None,
            classname: None,
            allow_null: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ArgType {}

#[derive(Debug)]
pub struct ArgInfo<'a> {
    name: &'a CStr,
    return_reference: bool,
    required_num_args: usize,
    r#type: Option<ArgType>,
    class_name: Option<&'a CStr>,
    allow_null: bool,
}

impl<'a> ArgInfo<'a> {
    pub fn new(name: &'a CStr) -> Self {
        Self {
            name,
            return_reference: false,
            required_num_args: 0,
            r#type: None,
            class_name: None,
            allow_null: false,
        }
    }
}
