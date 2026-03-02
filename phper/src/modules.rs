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
    classes::{ClassEntity, Interface, InterfaceEntity, StateClass},
    constants::Constant,
    errors::Throwable,
    functions::{Function, FunctionEntity, FunctionEntry, HandlerMap},
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
    sync::atomic::{AtomicPtr, Ordering},
};

/// Global pointer hold the Module builder.
/// Because PHP is single threaded, so there is no lock here.
static GLOBAL_MODULE: AtomicPtr<Module> = AtomicPtr::new(null_mut());

static GLOBAL_MODULE_ENTRY: AtomicPtr<zend_module_entry> = AtomicPtr::new(null_mut());

#[inline]
pub(crate) unsafe fn global_module<'a>() -> &'a Module {
    unsafe {
        let module = GLOBAL_MODULE.load(Ordering::Acquire);
        module.as_ref().unwrap()
    }
}

#[inline]
unsafe fn global_module_mut<'a>() -> &'a mut Module {
    unsafe {
        let module = GLOBAL_MODULE.load(Ordering::Acquire);
        module.as_mut().unwrap()
    }
}

#[cfg(phper_zts)]
type RequestHook = dyn Fn() + Send + Sync;

#[cfg(not(phper_zts))]
type RequestHook = dyn Fn();

unsafe extern "C" fn module_startup(_type: c_int, module_number: c_int) -> c_int {
    unsafe {
        let module = global_module_mut();

        ini::register(&module.ini_entities, module_number);

        for constant in &module.constants {
            constant.register(module_number);
        }

        for interface_entity in &module.interface_entities {
            interface_entity.init();
        }

        for function_entity in &module.function_entities {
            module.handler_map.insert(
                (None, function_entity.name.clone()),
                function_entity.handler.clone(),
            );
        }

        for class_entity in &module.class_entities {
            let ce = class_entity.init();
            class_entity.declare_properties(ce);
            module.handler_map.extend(class_entity.handler_map());
        }

        #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
        for enum_entity in &module.enum_entities {
            enum_entity.init();
            module.handler_map.extend(enum_entity.handler_map());
        }

        if let Some(f) = take(&mut module.module_init) {
            f();
        }

        ZEND_RESULT_CODE_SUCCESS
    }
}

unsafe extern "C" fn module_shutdown(_type: c_int, module_number: c_int) -> c_int {
    unsafe {
        let module = global_module_mut();

        ini::unregister(module_number);

        if let Some(f) = take(&mut module.module_shutdown) {
            f();
        }

        ZEND_RESULT_CODE_SUCCESS
    }
}

unsafe extern "C" fn request_startup(_type: c_int, _module_number: c_int) -> c_int {
    unsafe {
        let module = global_module();

        if let Some(f) = &module.request_init {
            f();
        }

        ZEND_RESULT_CODE_SUCCESS
    }
}

unsafe extern "C" fn request_shutdown(_type: c_int, _module_number: c_int) -> c_int {
    unsafe {
        let module = global_module();

        if let Some(f) = &module.request_shutdown {
            f();
        }

        ZEND_RESULT_CODE_SUCCESS
    }
}

unsafe extern "C" fn module_info(zend_module: *mut zend_module_entry) {
    unsafe {
        let module = global_module();

        php_info_print_table_start();
        if !module.version.as_bytes().is_empty() {
            php_info_print_table_row(2, c"version".as_ptr(), module.version.as_ptr());
        }
        if !module.author.as_bytes().is_empty() {
            php_info_print_table_row(2, c"authors".as_ptr(), module.author.as_ptr());
        }
        for (key, value) in &module.infos {
            php_info_print_table_row(2, key.as_ptr(), value.as_ptr());
        }
        php_info_print_table_end();

        display_ini_entries(zend_module);
    }
}

/// Builder for registering PHP Module.
#[allow(clippy::type_complexity)]
pub struct Module {
    name: CString,
    version: CString,
    author: CString,
    module_init: Option<Box<dyn FnOnce()>>,
    module_shutdown: Option<Box<dyn FnOnce()>>,
    request_init: Option<Box<RequestHook>>,
    request_shutdown: Option<Box<RequestHook>>,
    function_entities: Vec<FunctionEntity>,
    class_entities: Vec<ClassEntity<()>>,
    interface_entities: Vec<InterfaceEntity>,
    #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
    enum_entities: Vec<crate::enums::EnumEntity<()>>,
    constants: Vec<Constant>,
    ini_entities: Vec<ini::IniEntity>,
    infos: HashMap<CString, CString>,
    /// Used to find the handler in the invoke function.
    pub(crate) handler_map: HandlerMap,
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
            #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
            enum_entities: Default::default(),
            constants: Default::default(),
            ini_entities: Default::default(),
            infos: Default::default(),
            handler_map: Default::default(),
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
    #[cfg(phper_zts)]
    pub fn on_request_init(&mut self, func: impl Fn() + Send + Sync + 'static) {
        self.request_init = Some(Box::new(func));
    }

    /// Register `RINIT` hook.
    #[cfg(not(phper_zts))]
    pub fn on_request_init(&mut self, func: impl Fn() + 'static) {
        self.request_init = Some(Box::new(func));
    }

    /// Register `RSHUTDOWN` hook.
    #[cfg(phper_zts)]
    pub fn on_request_shutdown(&mut self, func: impl Fn() + Send + Sync + 'static) {
        self.request_shutdown = Some(Box::new(func));
    }

    /// Register `RSHUTDOWN` hook.
    #[cfg(not(phper_zts))]
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
    pub fn add_class<T>(&mut self, class: ClassEntity<T>) -> StateClass<T> {
        let bound_class = class.bound_class();
        self.class_entities
            .push(unsafe { transmute::<ClassEntity<T>, ClassEntity<()>>(class) });
        bound_class
    }

    /// Register interface to module.
    pub fn add_interface(&mut self, interface: InterfaceEntity) -> Interface {
        let bound_interface = interface.bound_interface();
        self.interface_entities.push(interface);
        bound_interface
    }

    /// Register enum to module.
    #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
    pub fn add_enum<B: crate::enums::EnumBackingType>(
        &mut self, enum_entity: crate::enums::EnumEntity<B>,
    ) -> crate::enums::Enum {
        let bound_enum = enum_entity.bound_enum();
        self.enum_entities.push(unsafe {
            transmute::<crate::enums::EnumEntity<B>, crate::enums::EnumEntity<()>>(enum_entity)
        });
        bound_enum
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
        unsafe {
            let entry = GLOBAL_MODULE_ENTRY.load(Ordering::Acquire);
            if !entry.is_null() {
                return entry;
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

            let module_ptr = Box::into_raw(module);
            let entry_ptr = Box::into_raw(entry);

            match GLOBAL_MODULE.compare_exchange(
                null_mut(),
                module_ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    GLOBAL_MODULE_ENTRY.store(entry_ptr, Ordering::Release);
                    entry_ptr
                }
                Err(_) => {
                    drop(Box::from_raw(module_ptr));
                    drop(Box::from_raw(entry_ptr));
                    loop {
                        let existing_entry = GLOBAL_MODULE_ENTRY.load(Ordering::Acquire);
                        if !existing_entry.is_null() {
                            break existing_entry;
                        }
                        std::hint::spin_loop();
                    }
                }
            }
        }
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

    #[inline]
    pub(crate) fn class_entities(&self) -> &[ClassEntity<()>] {
        &self.class_entities
    }
}
