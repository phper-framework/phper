use crate::{
    classes::Method,
    ini::create_ini_entry_ex,
    sys::{
        _zend_get_parameters_array_ex, phper_z_strval_p, phper_zval_zval, zend_arg_info,
        zend_ce_exception, zend_execute_data, zend_function_entry, zend_ini_entry_def,
        zend_internal_arg_info, zend_throw_exception, zend_uchar, zif_handler, zval,
    },
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
}

pub(crate) unsafe extern "C" fn invoke(
    execute_data: *mut zend_execute_data,
    return_value: *mut zval,
) {
    let execute_data = ExecuteData::from_mut(execute_data);
    let return_value = Val::from_mut(return_value);

    let num_args = execute_data.common_num_args();
    let arg_info = execute_data.common_arg_info();

    let last_arg_info = arg_info.offset(num_args as isize);
    let handler = (*last_arg_info).name as *const Box<dyn Function>;
    let handler = handler.as_ref().expect("handler is null");

    // TODO Do num args check

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

    #[cfg(any(phper_php_version = "7.1", phper_php_version = "7.0",))]
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
