use crate::sys::{zend_execute_data, zend_function_entry, zval};
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_uchar;
use std::ptr::null;

#[derive(Debug)]
pub struct Functions<'a> {
    pub(crate) inner: Vec<Function<'a>>,
}

impl<'a> Functions<'a> {
    #[inline]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    #[inline]
    pub fn new(inner: Vec<Function<'a>>) -> Self {
        Self { inner }
    }

    pub fn into_boxed_entries(self) -> Box<[zend_function_entry]> {
        let functions = self.inner;

        let mut entries = Vec::with_capacity(functions.len() + 1);

        for function in functions {
            entries.push(zend_function_entry {
                fname: function.name.as_ptr(),
                handler: Some(function.func),
                arg_info: null(),
                num_args: 0,
                flags: 0,
            });
        }

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

#[derive(Debug)]
pub struct Function<'a> {
    pub(crate) name: &'a CStr,
    pub(crate) func: extern "C" fn(*mut zend_execute_data, *mut zval),
    pub(crate) arg_info: Option<ArgInfo<'a>>,
    pub(crate) flags: u32,
}

impl<'a> Function<'a> {
    #[inline]
    pub fn new(name: &'a CStr, func: extern "C" fn(*mut zend_execute_data, *mut zval)) -> Self {
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
