use crate::{
    sys::{
        zend_function_entry, zend_module_entry, zend_register_ini_entries,
        zend_unregister_ini_entries, PHP_MODULE_BUILD_ID, USING_ZTS, ZEND_DEBUG,
        ZEND_MODULE_API_NO,
    },
    zend::ini::IniEntries,
};
use std::{
    cell::Cell,
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void},
    ptr::{null, null_mut},
};

pub struct ModuleEntryBuilder {
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
}

impl ModuleEntryBuilder {
    pub const fn new(name: *const c_char, version: *const c_char) -> Self {
        Self {
            name,
            version,
            functions: null(),
            module_startup_func: None,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: None,
            globals_ctor: None,
            globals_dtor: None,
        }
    }

    pub const fn functions(self, functions: *const zend_function_entry) -> Self {
        Self { functions, ..self }
    }

    pub const fn module_startup_func(
        self,
        module_startup_func: unsafe extern "C" fn(c_int, c_int) -> c_int,
    ) -> Self {
        Self {
            module_startup_func: Some(module_startup_func),
            ..self
        }
    }

    pub const fn module_shutdown_func(
        self,
        module_shutdown_func: unsafe extern "C" fn(c_int, c_int) -> c_int,
    ) -> Self {
        Self {
            module_shutdown_func: Some(module_shutdown_func),
            ..self
        }
    }

    pub const fn request_startup_func(
        self,
        request_startup_func: unsafe extern "C" fn(c_int, c_int) -> c_int,
    ) -> Self {
        Self {
            request_startup_func: Some(request_startup_func),
            ..self
        }
    }

    pub const fn request_shutdown_func(
        self,
        request_shutdown_func: unsafe extern "C" fn(c_int, c_int) -> c_int,
    ) -> Self {
        Self {
            request_shutdown_func: Some(request_shutdown_func),
            ..self
        }
    }

    pub const fn info_func(self, info_func: unsafe extern "C" fn(*mut zend_module_entry)) -> Self {
        Self {
            info_func: Some(info_func),
            ..self
        }
    }

    pub const fn globals_ctor(
        self,
        globals_ctor: unsafe extern "C" fn(global: *mut c_void),
    ) -> Self {
        Self {
            globals_ctor: Some(globals_ctor),
            ..self
        }
    }

    pub const fn globals_dtor(
        self,
        globals_dtor: unsafe extern "C" fn(global: *mut c_void),
    ) -> Self {
        Self {
            globals_dtor: Some(globals_dtor),
            ..self
        }
    }

    pub const fn build(self) -> ModuleEntry {
        ModuleEntry::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: self.name,
            functions: self.functions,
            module_startup_func: self.module_startup_func,
            module_shutdown_func: self.module_shutdown_func,
            request_startup_func: self.request_startup_func,
            request_shutdown_func: self.request_shutdown_func,
            info_func: self.info_func,
            version: self.version,
            globals_size: 0usize,
            #[cfg(phper_zts)]
            globals_id_ptr: std::ptr::null_mut(),
            #[cfg(not(phper_zts))]
            globals_ptr: std::ptr::null_mut(),
            globals_ctor: self.globals_ctor,
            globals_dtor: self.globals_dtor,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: PHP_MODULE_BUILD_ID,
        })
    }
}

#[repr(transparent)]
pub struct ModuleEntry {
    raw: Cell<zend_module_entry>,
}

impl ModuleEntry {
    pub const fn new(raw: zend_module_entry) -> Self {
        Self {
            raw: Cell::new(raw),
        }
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
    _type_: c_int,
    module_number: c_int,
}

impl ModuleArgs {
    pub const fn new(_type_: c_int, module_number: c_int) -> Self {
        Self {
            _type_,
            module_number,
        }
    }

    pub fn register_ini_entries<const N: usize>(&self, ini_entries: &IniEntries<N>) {
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
