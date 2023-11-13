// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_module_entry].

use crate::{
    c_str_ptr,
    classes::{ClassEntity, InterfaceEntity},
    constants::Constant,
    errors::Throwable,
    functions::{Function, FunctionEntity, FunctionEntry},
    ini,
    sys::*,
    types::Scalar,
    utils::ensure_end_with_zero,
    values::ZVal,
};
use std::{
    collections::HashMap,
    ffi::CString,
    mem::{size_of, take, transmute, zeroed},
    os::raw::{c_int, c_uchar, c_uint, c_ushort},
    ptr::{null, null_mut},
    rc::Rc,
};

/// Global pointer hold the Module builder.
/// Because PHP is single threaded, so there is no lock here.
static mut GLOBAL_MODULE: *mut Module = null_mut();

static mut GLOBAL_MODULE_ENTRY: *mut zend_module_entry = null_mut();

unsafe extern "C" fn module_startup(_type: c_int, module_number: c_int) -> c_int {
    let module = GLOBAL_MODULE.as_mut().unwrap();

    ini::register(&module.ini_entities, module_number);

    for constant in &module.constants {
        constant.register(module_number);
    }

    for class_entity in &module.class_entities {
        let ce = class_entity.init();
        class_entity.declare_properties(ce);
    }

    for interface_entity in &module.interface_entities {
        interface_entity.init();
    }

    if let Some(f) = take(&mut module.module_init) {
        f();
    }

    ZEND_RESULT_CODE_SUCCESS
}

unsafe extern "C" fn module_shutdown(_type: c_int, module_number: c_int) -> c_int {
    let module = GLOBAL_MODULE.as_mut().unwrap();

    ini::unregister(module_number);

    if let Some(f) = take(&mut module.module_shutdown) {
        f();
    }

    ZEND_RESULT_CODE_SUCCESS
}

unsafe extern "C" fn request_startup(_type: c_int, _module_number: c_int) -> c_int {
    let module = GLOBAL_MODULE.as_ref().unwrap();

    if let Some(f) = &module.request_init {
        f();
    }

    ZEND_RESULT_CODE_SUCCESS
}

unsafe extern "C" fn request_shutdown(_type: c_int, _module_number: c_int) -> c_int {
    let module = GLOBAL_MODULE.as_ref().unwrap();

    if let Some(f) = &module.request_shutdown {
        f();
    }

    ZEND_RESULT_CODE_SUCCESS
}

unsafe extern "C" fn module_info(zend_module: *mut zend_module_entry) {
    let module = GLOBAL_MODULE.as_ref().unwrap();

    php_info_print_table_start();
    if !module.version.as_bytes().is_empty() {
        php_info_print_table_row(2, c_str_ptr!("version"), module.version.as_ptr());
    }
    if !module.author.as_bytes().is_empty() {
        php_info_print_table_row(2, c_str_ptr!("authors"), module.author.as_ptr());
    }
    for (key, value) in &module.infos {
        php_info_print_table_row(2, key.as_ptr(), value.as_ptr());
    }
    php_info_print_table_end();

    display_ini_entries(zend_module);
}

/// Builder for registering PHP Module.
#[allow(clippy::type_complexity)]
pub struct Module {
    name: CString,
    version: CString,
    author: CString,
    module_init: Option<Box<dyn FnOnce()>>,
    module_shutdown: Option<Box<dyn FnOnce()>>,
    request_init: Option<Box<dyn Fn()>>,
    request_shutdown: Option<Box<dyn Fn()>>,
    function_entities: Vec<FunctionEntity>,
    class_entities: Vec<ClassEntity<()>>,
    interface_entities: Vec<InterfaceEntity>,
    constants: Vec<Constant>,
    ini_entities: Vec<ini::IniEntity>,
    infos: HashMap<CString, CString>,
}

impl Module {
    /// Construct the `Module` with base metadata.
    pub fn new(
        name: impl Into<String>, version: impl Into<String>, author: impl Into<String>,
    ) -> Self {
        Self {
            name: ensure_end_with_zero(name),
            version: ensure_end_with_zero(version),
            author: ensure_end_with_zero(author),
            module_init: None,
            module_shutdown: None,
            request_init: None,
            request_shutdown: None,
            function_entities: vec![],
            class_entities: Default::default(),
            interface_entities: Default::default(),
            constants: Default::default(),
            ini_entities: Default::default(),
            infos: Default::default(),
        }
    }

    /// Register `MINIT` hook.
    pub fn on_module_init(&mut self, func: impl FnOnce() + 'static) {
        self.module_init = Some(Box::new(func));
    }

    /// Register `MSHUTDOWN` hook.
    pub fn on_module_shutdown(&mut self, func: impl FnOnce() + 'static) {
        self.module_shutdown = Some(Box::new(func));
    }

    /// Register `RINIT` hook.
    pub fn on_request_init(&mut self, func: impl Fn() + 'static) {
        self.request_init = Some(Box::new(func));
    }

    /// Register `RSHUTDOWN` hook.
    pub fn on_request_shutdown(&mut self, func: impl Fn() + 'static) {
        self.request_shutdown = Some(Box::new(func));
    }

    /// Register function to module.
    pub fn add_function<F, Z, E>(
        &mut self, name: impl Into<String>, handler: F,
    ) -> &mut FunctionEntity
    where
        F: Fn(&mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        self.function_entities
            .push(FunctionEntity::new(name, Rc::new(Function::new(handler))));
        self.function_entities.last_mut().unwrap()
    }

    /// Register class to module.
    pub fn add_class<T>(&mut self, class: ClassEntity<T>) {
        self.class_entities.push(unsafe { transmute(class) });
    }

    /// Register interface to module.
    pub fn add_interface(&mut self, interface: InterfaceEntity) {
        self.interface_entities.push(interface);
    }

    /// Register constant to module.
    pub fn add_constant(&mut self, name: impl Into<String>, value: impl Into<Scalar>) {
        self.constants.push(Constant::new(name, value));
    }

    /// Register ini configuration to module.
    pub fn add_ini(
        &mut self, name: impl Into<String>, default_value: impl ini::IntoIniValue,
        policy: ini::Policy,
    ) {
        self.ini_entities
            .push(ini::IniEntity::new(name, default_value, policy));
    }

    /// Register info item.
    ///
    /// # Panics
    ///
    /// Panic if key or value contains '\0'.
    pub fn add_info(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = CString::new(key.into()).expect("key contains '\0'");
        let value = CString::new(value.into()).expect("value contains '\0'");
        self.infos.insert(key, value);
    }

    /// Leak memory to generate `zend_module_entry` pointer.
    #[doc(hidden)]
    pub unsafe fn module_entry(self) -> *const zend_module_entry {
        if !GLOBAL_MODULE_ENTRY.is_null() {
            return GLOBAL_MODULE_ENTRY;
        }

        assert!(!self.name.as_bytes().is_empty(), "module name must be set");
        assert!(
            !self.version.as_bytes().is_empty(),
            "module version must be set"
        );

        let module = Box::new(self);

        let entry: Box<zend_module_entry> = Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: module.name.as_ptr(),
            functions: module.function_entries(),
            module_startup_func: Some(module_startup),
            module_shutdown_func: Some(module_shutdown),
            request_startup_func: Some(request_startup),
            request_shutdown_func: Some(request_shutdown),
            info_func: Some(module_info),
            version: module.version.as_ptr(),
            globals_size: 0,
            #[cfg(phper_zts)]
            globals_id_ptr: std::ptr::null_mut(),
            #[cfg(not(phper_zts))]
            globals_ptr: std::ptr::null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: phper_get_zend_module_build_id(),
        });

        GLOBAL_MODULE = Box::into_raw(module);
        GLOBAL_MODULE_ENTRY = Box::into_raw(entry);

        GLOBAL_MODULE_ENTRY
    }

    fn function_entries(&self) -> *const zend_function_entry {
        if self.function_entities.is_empty() {
            return null();
        }

        let mut entries = Vec::new();
        for f in &self.function_entities {
            entries.push(unsafe { FunctionEntry::from_function_entity(f) });
        }
        entries.push(unsafe { zeroed::<zend_function_entry>() });

        Box::into_raw(entries.into_boxed_slice()).cast()
    }
}
