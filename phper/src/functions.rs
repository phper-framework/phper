use crate::{
    classes::Method,
    ini::create_ini_entry_ex,
    sys::*,
    throws::Throwable,
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

#[repr(transparent)]
pub struct FunctionEntry {
    inner: zend_function_entry,
}

pub(crate) struct FunctionEntity {
    pub(crate) name: String,
    pub(crate) handler: Box<dyn Function>,
    pub(crate) arguments: Vec<Argument>,
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

    // TODO I don't know why this field is zero.
    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let mut num_args = 0isize;
    for i in 0..10isize {
        let buf = transmute::<_, [u8; size_of::<zend_arg_info>()]>(*arg_info.offset(i as isize));
        if buf == zeroed::<[u8; size_of::<zend_arg_info>()]>() {
            num_args = i;
            break;
        }
    }
    if num_args == 0 {
        unreachable!();
    }
    num_args += 1;

    let last_arg_info = arg_info.offset(num_args as isize);
    let handler = (*last_arg_info).name as *const Box<dyn Function>;
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

    handler.call(&mut arguments, return_value);
}

pub(crate) unsafe extern "C" fn method_invoke(
    execute_data: *mut zend_execute_data,
    return_value: *mut zval,
) {
    let execute_data = ExecuteData::from_mut(execute_data);
    let return_value = Val::from_mut(return_value);

    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let last_arg_info = arg_info.offset(num_args as isize);
    let handler = (*last_arg_info).name as *const Box<dyn Method>;
    let handler = handler.as_ref().expect("handler is null");

    // TODO Do num args check

    let mut arguments = execute_data.get_parameters_array();

    handler.call(execute_data.get_this(), &mut arguments, return_value);
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
