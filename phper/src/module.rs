use crate::sys::{
    zend_function_entry, zend_module_entry, USING_ZTS, ZEND_DEBUG,
    ZEND_MODULE_API_NO,
};
use crate::{functions_into_boxed_entries, Function, FunctionArray};
use std::ffi::CStr;

use std::mem::size_of;
use std::os::raw::{c_uchar, c_uint, c_ushort};
use std::ptr::{null, null_mut};
use thiserror::Error;

#[derive(Default)]
pub struct Module<'a> {
    pub name: &'a CStr,
    pub version: &'a CStr,
    pub functions: Option<Vec<Function<'a>>>,
}

impl<'a> Module<'a> {
    fn into_boxed_entry(self) -> Box<zend_module_entry> {
        let functions = self.functions.unwrap_or_else(|| Vec::new());
        let functions =
            Box::into_raw(functions_into_boxed_entries(&functions)) as *const zend_function_entry;

        Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: self.name.as_ptr(),
            functions,
            module_startup_func: None,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: None,
            version: self.version.as_ptr(),
            globals_size: 0usize,
            globals_ptr: null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: PHP_EXTENSION_BUILD,
        })
    }
}

impl From<Module<'_>> for *const zend_module_entry {
    fn from(module: Module<'_>) -> Self {
        Box::into_raw(module.into_boxed_entry())
    }
}
