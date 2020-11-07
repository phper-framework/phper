use crate::sys::zend_module_entry;
use std::cell::Cell;
use std::mem::size_of;
use std::os::raw::{c_ushort, c_uint, c_uchar, c_char, c_int, c_void};
use crate::sys::ZEND_MODULE_API_NO;
use crate::sys::ZEND_DEBUG;
use crate::sys::USING_ZTS;
use crate::sys::PHP_MODULE_BUILD_ID;
use crate::sys::zend_function_entry;
use std::ptr::{null, null_mut};
use crate::zend::ini::IniEntryDefs;
use crate::sys::zend_register_ini_entries;
use crate::sys::zend_unregister_ini_entries;

pub const fn create_zend_module_entry(
    name: *const c_char,
    version: *const c_char,
    functions: *const zend_function_entry,
    module_startup_func: Option<unsafe extern "C" fn(c_int, c_int) -> c_int>,
    module_shutdown_func: Option<unsafe extern "C" fn(c_int, c_int) -> c_int>,
    request_startup_func: Option<unsafe extern "C" fn(c_int, c_int) -> c_int>,
    request_shutdown_func: Option<unsafe extern "C" fn(c_int, c_int) -> c_int>,
    info_func: Option<unsafe extern "C" fn(*mut zend_module_entry)>,
    globals_ctor: Option<unsafe extern "C" fn(global: *mut c_void)>,
    globals_dtor: Option<unsafe extern "C" fn(global: *mut c_void)>,
) -> zend_module_entry {
    zend_module_entry {
        size: size_of::<zend_module_entry>() as c_ushort,
        zend_api: ZEND_MODULE_API_NO as c_uint,
        zend_debug: ZEND_DEBUG as c_uchar,
        zts: USING_ZTS as c_uchar,
        ini_entry: null(),
        deps: null(),
        name,
        functions,
        module_startup_func,
        module_shutdown_func,
        request_startup_func,
        request_shutdown_func,
        info_func,
        version,
        globals_size: 0usize,
        #[cfg(phper_zts)]
        globals_id_ptr: std::ptr::null_mut(),
        #[cfg(not(phper_zts))]
        globals_ptr: std::ptr::null_mut(),
        globals_ctor,
        globals_dtor,
        post_deactivate_func: None,
        module_started: 0,
        type_: 0,
        handle: null_mut(),
        module_number: 0,
        build_id: PHP_MODULE_BUILD_ID,
    }
}

#[repr(transparent)]
pub struct ModuleEntry {
    raw: Cell<zend_module_entry>,
}

impl ModuleEntry {
    pub const fn new(raw: zend_module_entry) -> Self {
        Self { raw: Cell::new(raw) }
    }

    pub const fn as_ptr(&self) -> *mut zend_module_entry {
        self.raw.as_ptr()
    }

    pub fn from_ptr<'a>(ptr: *const zend_module_entry) -> &'a Self {
        unsafe { &*(ptr as *const Self) }
    }
}

unsafe impl Sync for ModuleEntry {}

pub struct ModuleArgs {
    type_: c_int,
    module_number: c_int,
}

impl ModuleArgs {
    pub const fn new(type_: c_int, module_number: c_int) -> Self {
        Self {
            type_,
            module_number
        }
    }

    pub fn register_ini_entries<const N: usize>(&self, ini_entries: &IniEntryDefs<N>) {
        unsafe {
            zend_register_ini_entries(ini_entries.as_ptr(), self.module_number);
        }
    }

    pub fn unregister_ini_entries(&self) {
        unsafe {
            zend_unregister_ini_entries(self.module_number);
        }
    }
}

