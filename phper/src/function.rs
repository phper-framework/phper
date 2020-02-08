use crate::sys::{zend_execute_data, zend_function_entry, zval};
use crate::{FunctionType, Parameters, Value};
use derive_builder::Builder;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_uchar;
use std::ptr::null;

pub struct Functions<'a> {
    pub(crate) inner: Vec<Function<'a>>,
}

impl<'a> Functions<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::from_vec_function(Vec::new())
    }

    #[inline]
    pub fn from_vec_function(inner: Vec<Function<'a>>) -> Self {
        Self { inner }
    }

    pub fn into_boxed_entries(self) -> Box<[zend_function_entry]> {
        let functions = self.inner;

        let mut entries = Vec::with_capacity(functions.len() + 1);

        //        for function in functions {
        //            entries.push(zend_function_entry {
        //                fname: function.name.as_ptr(),
        //                handler: Some(function.func),
        //                arg_info: null(),
        //                num_args: 0,
        //                flags: 0,
        //            });
        //        }

        entries.push(zend_function_entry::default());

        entries.into_boxed_slice()
    }
}

impl<'a> Deref for Functions<'a> {
    type Target = Vec<Function<'a>>;

    #[inline]
    fn deref(&self) -> &Vec<Function<'a>> {
        &self.inner
    }
}

impl<'a> DerefMut for Functions<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Vec<Function<'a>> {
        &mut self.inner
    }
}

pub struct Function<'a> {
    pub(crate) name: &'a CStr,
    pub(crate) func: FunctionType<'a>,
    pub(crate) arg_info: Option<ArgInfo<'a>>,
    pub(crate) flags: u32,
}

impl<'a> Function<'a> {
    #[inline]
    pub fn new(name: &'a CStr, func: FunctionType<'a>) -> Self {
        Self {
            name,
            func,
            arg_info: None,
            flags: 0,
        }
    }

    pub fn arg_info(mut self, arg_info: ArgInfo<'a>) -> Self {
        self.arg_info = Some(arg_info);
        self
    }
}

#[derive(Debug)]
pub struct BeginArgInfo<'a> {
    pass_by_ref: c_uchar,
    type_hint: Option<ArgType>,
    classname: Option<&'a CStr>,
    allow_null: bool,
}

impl<'a> BeginArgInfo<'a> {
    pub fn new(name: &'a CStr) -> Self {
        Self {
            pass_by_ref: 0,
            type_hint: None,
            classname: None,
            allow_null: false,
        }
    }
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

    #[builder(default)]
    return_reference: bool,

    #[builder(default)]
    required_num_args: usize,

    r#type: Option<ArgType>,

    #[builder(default)]
    class_name: Option<&'a CStr>,

    #[builder(default)]
    allow_null: bool,
}

impl<'a> ArgInfo<'a> {
    pub fn builder() -> ArgInfoBuilder<'a> {
        Default::default()
    }
}
