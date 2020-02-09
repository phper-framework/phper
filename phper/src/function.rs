use crate::sys::FunctionHandler;
use crate::sys::{zend_execute_data, zend_function_entry, zval};

use derive_builder::Builder;
use std::ffi::CStr;

use std::os::raw::c_uchar;
use std::ptr::null;

pub(crate) fn functions_into_boxed_entries(functions: Functions) -> Box<[zend_function_entry]> {
    let mut entries = Vec::with_capacity(functions.len() + 1);

    for function in functions {
        entries.push(zend_function_entry {
            fname: function.name.as_ptr(),
            handler: Some(function.handler),
            arg_info: null(),
            num_args: 0,
            flags: 0,
        });
    }

    entries.push(zend_function_entry::default());

    entries.into_boxed_slice()
}

pub(crate) type Functions<'a> = &'a [Function<'a>];

#[derive(Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct Function<'a> {
    pub(crate) name: &'a CStr,

    pub(crate) handler: FunctionHandler,

    #[builder(default)]
    pub(crate) arg_info: Option<ArgInfo<'a>>,

    #[builder(default)]
    pub(crate) flags: u32,
}

impl<'a> Function<'a> {
    pub fn builder() -> FunctionBuilder<'a> {
        Default::default()
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(strip_option), build_fn(skip))]
pub struct BeginArgInfo<'a> {
    pass_by_ref: c_uchar,
    type_hint: Option<ArgType>,
    classname: Option<&'a CStr>,
    allow_null: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum ArgType {
    Undef = 0,
    Null = 1,
    False = 2,
    True = 3,
    Long = 4,
    Double = 5,
    String = 6,
    Array = 7,
    Object = 8,
    Resource = 9,
    Reference = 10,
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(strip_option), build_fn(skip))]
pub struct ArgInfo<'a> {
    name: &'a CStr,

    return_reference: bool,

    required_num_args: usize,

    r#type: Option<ArgType>,

    class_name: Option<&'a CStr>,

    allow_null: bool,
}

impl<'a> ArgInfo<'a> {
    pub fn builder() -> ArgInfoBuilder<'a> {
        Default::default()
    }
}
