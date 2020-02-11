use crate::sys::{zend_execute_data, zend_function_entry, zval, InternalRawFunction, zend_internal_arg_info};

use std::ffi::CStr;

use std::os::raw::{c_uchar, c_char};
use std::ptr::{null, null_mut};

pub(crate) fn functions_into_boxed_entries(functions: FunctionArray) -> Box<[zend_function_entry]> {
    let mut entries = Vec::with_capacity(functions.len() + 1);

    for function in functions {
        entries.push(zend_function_entry {
            fname: function.name.as_ptr(),
            handler: Some(function.handler.clone().into()),
            arg_info: function.arg_info.as_ref().map(|arg_info| arg_info.into()).unwrap_or(null()),
            num_args: function.arg_info.as_ref().map(|arg_info| arg_info.parameters.len() as u32).unwrap_or(0),
            flags: 0,
        });
    }

    entries.push(zend_function_entry::default());

    entries.into_boxed_slice()
}

pub(crate) type FunctionArray<'a> = &'a [Function<'a>];

#[derive(Default)]
pub struct Function<'a> {
    pub name: &'a CStr,
    pub handler: FunctionHandler,
    pub arg_info: Option<InternalArgInfoArray<'a>>,
    pub flags: u32,
}

#[derive(Debug)]
pub struct InternalArgInfoArray<'a> {
    pub begin: InternalBeginArgInfo<'a>,
    pub parameters: Vec<InternalArgInfo<'a>>,
}

impl Into<*const zend_internal_arg_info> for &InternalArgInfoArray<'_> {
    fn into(self) -> *const zend_internal_arg_info {
        let mut infos: Vec<*const zend_internal_arg_info> = Vec::with_capacity(self.parameters.len() + 1);
        let begin: *const zend_internal_arg_info = Box::into_raw(Box::new((&self.begin).into()));
        infos.push(begin);
        for parameter in &self.parameters {
            let parameter: *const zend_internal_arg_info = Box::into_raw(Box::new(parameter.into()));
            infos.push(parameter);
        }
        Box::into_raw(infos.into_boxed_slice()) as *const zend_internal_arg_info
    }
}

#[derive(Debug, Default)]
pub struct InternalArgInfo<'a> {
    pub name: &'a CStr,
    pub pass_by_ref: bool,
    pub type_hint: ArgType,
    pub class_name: Option<&'a CStr>,
    pub allow_null: bool,
}

impl Into<zend_internal_arg_info> for &InternalArgInfo<'_> {
    fn into(self) -> zend_internal_arg_info {
        zend_internal_arg_info {
            name: self.name.as_ptr(),
            class_name: self.class_name.map(|class_name| class_name.as_ptr()).unwrap_or(null()),
            type_hint: self.type_hint as c_uchar,
            pass_by_reference: self.pass_by_ref as c_uchar,
            allow_null: self.allow_null as c_uchar,
            is_variadic: 0 as c_uchar,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
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

impl Default for ArgType {
    fn default() -> Self {
        ArgType::Undef
    }
}

#[derive(Debug, Default)]
pub struct InternalBeginArgInfo<'a> {
    pub return_reference: bool,
    pub required_num_args: usize,
    pub type_hint: ArgType,
    pub class_name: Option<&'a CStr>,
    pub allow_null: bool,
}

impl Into<zend_internal_arg_info> for &InternalBeginArgInfo<'_> {
    fn into(self) -> zend_internal_arg_info {
        zend_internal_arg_info {
            name: self.required_num_args as *const c_char,
            class_name: self.class_name.map(|class_name| class_name.as_ptr()).unwrap_or(null()),
            type_hint: self.type_hint as c_uchar,
            pass_by_reference: self.return_reference as c_uchar,
            allow_null: self.allow_null as c_uchar,
            is_variadic: 0 as c_uchar,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub enum FunctionHandler {
    Internal(InternalRawFunction),
}

extern "C" fn null_func(execute_data: *mut zend_execute_data, return_value: *mut zval) {}

impl Default for FunctionHandler {
    fn default() -> Self {
        FunctionHandler::Internal(null_func)
    }
}

impl From<FunctionHandler> for InternalRawFunction {
    fn from(fh: FunctionHandler) -> Self {
        match fh {
            FunctionHandler::Internal(irf) => irf,
            _ => todo!(),
        }
    }
}
