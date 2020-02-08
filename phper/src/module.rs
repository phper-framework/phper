use crate::sys::{
    c_str, zend_function_entry, zend_module_entry, PHP_EXTENSION_BUILD, USING_ZTS, ZEND_DEBUG,
    ZEND_MODULE_API_NO,
};
use crate::Functions;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_uchar, c_uint, c_ushort};
use std::ptr::{null, null_mut};

pub struct Module<'a> {
    name: &'a CStr,
    version: &'a CStr,
    functions: Option<Functions<'a>>,
}

impl<'a> Module<'a> {
    #[inline]
    pub fn new(name: &'a CStr, version: &'a CStr) -> Self {
        Self {
            name,
            version,
            functions: None,
        }
    }

    pub fn name(mut self, name: &'a CStr) -> Self {
        self.name = name;
        self
    }

    pub fn version(mut self, version: &'a CStr) -> Self {
        self.version = version;
        self
    }

    pub fn functions(mut self, functions: Functions<'a>) -> Self {
        self.functions = Some(functions);
        self
    }

    pub fn into_boxed_entry(self) -> Box<zend_module_entry> {
        let functions = self.functions.unwrap_or_else(|| Functions::new());
        let functions = Box::into_raw(functions.into_boxed_entries()) as *const zend_function_entry;

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
