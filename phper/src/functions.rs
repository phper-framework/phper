use crate::{
    classes::This,
    errors::Throwable,
    ini::create_ini_entry_ex,
    sys::*,
    values::{ExecuteData, SetVal, Val},
};
use std::{
    cell::Cell,
    ffi::CStr,
    mem::{size_of, transmute, zeroed},
    os::raw::{c_char, c_int},
    ptr::{null, null_mut},
};

pub trait Function: Send + Sync {
    fn call(&self, arguments: &mut [Val], return_value: &mut Val);
}

impl<F, R> Function for F
where
    F: Fn(&mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, arguments: &mut [Val], return_value: &mut Val) {
        let r = self(arguments);
        r.set_val(return_value);
    }
}

pub trait Method: Send + Sync {
    fn call(&self, this: &mut This, arguments: &mut [Val], return_value: &mut Val);
}

impl<F, R> Method for F
where
    F: Fn(&mut This, &mut [Val]) -> R + Send + Sync,
    R: SetVal,
{
    fn call(&self, this: &mut This, arguments: &mut [Val], return_value: &mut Val) {
        let r = self(this, arguments);
        r.set_val(return_value);
    }
}

pub(crate) enum Callable {
    Function(Box<dyn Function>),
    Method(Box<dyn Method>),
}

#[repr(transparent)]
pub struct FunctionEntry {
    inner: zend_function_entry,
}

pub struct FunctionEntity {
    pub(crate) name: String,
    pub(crate) handler: Callable,
    pub(crate) arguments: Vec<Argument>,
}

impl FunctionEntity {
    pub(crate) fn new(name: impl ToString, handler: Callable, arguments: Vec<Argument>) -> Self {
        let mut name = name.to_string();
        name.push('\0');
        FunctionEntity {
            name,
            handler,
            arguments,
        }
    }

    // Leak memory
    pub(crate) unsafe fn entry(&self) -> zend_function_entry {
        let mut infos = Vec::new();

        let require_arg_count = self.arguments.iter().filter(|arg| arg.required).count();
        infos.push(create_zend_arg_info(
            require_arg_count as *const c_char,
            false,
        ));

        for arg in &self.arguments {
            infos.push(create_zend_arg_info(
                arg.name.as_ptr().cast(),
                arg.pass_by_ref,
            ));
        }

        infos.push(unsafe { zeroed::<zend_internal_arg_info>() });

        let mut last_arg_info = unsafe { zeroed::<zend_internal_arg_info>() };
        last_arg_info.name = ((&self.handler) as *const _ as *mut i8).cast();
        infos.push(last_arg_info);

        zend_function_entry {
            fname: self.name.as_ptr().cast(),
            handler: Some(invoke),
            arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
            num_args: self.arguments.len() as u32,
            flags: 0,
        }
    }
}

pub struct Argument {
    pub(crate) name: String,
    pub(crate) pass_by_ref: bool,
    pub(crate) required: bool,
}

impl Argument {
    pub fn by_val(name: impl ToString) -> Self {
        let mut name = name.to_string();
        name.push('\0');
        Self {
            name,
            pass_by_ref: false,
            required: true,
        }
    }

    pub fn by_ref(name: impl ToString) -> Self {
        let mut name = name.to_string();
        name.push('\0');
        Self {
            name,
            pass_by_ref: true,
            required: true,
        }
    }

    pub fn by_val_optional(name: impl ToString) -> Self {
        let mut name = name.to_string();
        name.push('\0');
        Self {
            name,
            pass_by_ref: false,
            required: false,
        }
    }

    pub fn by_ref_optional(name: impl ToString) -> Self {
        let mut name = name.to_string();
        name.push('\0');
        Self {
            name,
            pass_by_ref: true,
            required: false,
        }
    }
}

pub(crate) unsafe extern "C" fn invoke(
    execute_data: *mut zend_execute_data,
    return_value: *mut zval,
) {
    let execute_data = ExecuteData::from_mut(execute_data);
    let return_value = Val::from_mut(return_value);

    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let last_arg_info = arg_info.offset((num_args + 1) as isize);
    let handler = (*last_arg_info).name as *const Callable;
    let handler = handler.as_ref().expect("handler is null");

    // Check arguments count.
    if execute_data.num_args() < execute_data.common_required_num_args() {
        let s = format!(
            "expects at least {} parameter(s), {} given\0",
            execute_data.common_required_num_args(),
            execute_data.num_args()
        );
        php_error_docref(null(), E_WARNING as i32, s.as_ptr().cast());
        ().set_val(return_value);
        return;
    }

    let mut arguments = execute_data.get_parameters_array();

    match handler {
        Callable::Function(f) => {
            f.call(&mut arguments, return_value);
        }
        Callable::Method(m) => {
            m.call(execute_data.get_this(), &mut arguments, return_value);
        }
    }
}

pub const fn create_zend_arg_info(
    name: *const c_char,
    pass_by_ref: bool,
) -> zend_internal_arg_info {
    #[cfg(any(
        phper_php_version = "8.0",
        phper_php_version = "7.4",
        phper_php_version = "7.3",
        phper_php_version = "7.2"
    ))]
    {
        zend_internal_arg_info {
            name,
            type_: 0 as crate::sys::zend_type,
            pass_by_reference: pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }

    #[cfg(any(phper_php_version = "7.1", phper_php_version = "7.0"))]
    {
        zend_internal_arg_info {
            name,
            class_name: std::ptr::null(),
            type_hint: 0,
            allow_null: 0,
            pass_by_reference: pass_by_ref as zend_uchar,
            is_variadic: 0,
        }
    }
}
