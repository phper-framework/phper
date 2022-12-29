// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_module_entry].

use crate::{
    arrays::ZArr,
    c_str_ptr,
    classes::ClassEntity,
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
    ffi::{CString, CStr},
    marker::PhantomData,
    mem::{size_of, take, transmute, zeroed, ManuallyDrop},
    os::raw::{c_int, c_uchar, c_uint, c_ushort},
    ptr::{null, null_mut},
    rc::Rc, sync::Mutex,
};
use once_cell::sync::Lazy;

unsafe extern "C" fn module_startup(_type: c_int, module_number: c_int) -> c_int {
    let module_entry = ModuleEntry::from_globals_mut(module_number);
    let module_name = module_entry.name().to_bytes().to_vec();
    let module = module_entry.get_module_mut();
    debug_assert_eq!(module_name, module.name.to_bytes());

    ini::register(&module.ini_entities, module_number);

    for constant in &module.constants {
        constant.register(module_number);
    }

    for class_entity in &module.class_entities {
        let ce = class_entity.init();
        class_entity.declare_properties(ce);
    }

    match take(&mut module.module_init) {
        Some(f) => f(module_entry) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn module_shutdown(_type: c_int, module_number: c_int) -> c_int {
    dbg!("module_shutdown");
    let module_entry = ModuleEntry::from_globals_mut(module_number);
    let module_name = module_entry.name().to_bytes().to_vec();
    let module = module_entry.get_module_mut();
    debug_assert_eq!(module_name, module.name.to_bytes());

    ini::unregister(module_number);

    match take(&mut module.module_shutdown) {
        Some(f) => f(module_entry) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn request_startup(_type: c_int, module_number: c_int) -> c_int {
    let module_entry = ModuleEntry::from_globals(module_number);
    let module = module_entry.get_module();

    match &module.request_init {
        Some(f) => f(module_entry) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn request_shutdown(_type: c_int, module_number: c_int) -> c_int {
    let module_entry = ModuleEntry::from_globals(module_number);
    let module = module_entry.get_module();

    match &module.request_shutdown {
        Some(f) => f(module_entry) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn module_info(zend_module: *mut zend_module_entry) {
    let module_entry = ModuleEntry::from_ptr(zend_module);
    let module = module_entry.get_module();

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
    module_init: Option<Box<dyn FnOnce(&ModuleEntry) -> bool + Send + Sync>>,
    module_shutdown: Option<Box<dyn FnOnce(&ModuleEntry) -> bool + Send + Sync>>,
    request_init: Option<Box<dyn Fn(&ModuleEntry) -> bool + Send + Sync>>,
    request_shutdown: Option<Box<dyn Fn(&ModuleEntry) -> bool + Send + Sync>>,
    function_entities: Vec<FunctionEntity>,
    class_entities: Vec<ClassEntity<()>>,
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
            constants: Default::default(),
            ini_entities: Default::default(),
            infos: Default::default(),
        }
    }

    /// Register `MINIT` hook.
    pub fn on_module_init(
        &mut self, func: impl FnOnce(&ModuleEntry) -> bool + Send + Sync + 'static,
    ) {
        self.module_init = Some(Box::new(func));
    }

    /// Register `MSHUTDOWN` hook.
    pub fn on_module_shutdown(
        &mut self, func: impl FnOnce(&ModuleEntry) -> bool + Send + Sync + 'static,
    ) {
        self.module_shutdown = Some(Box::new(func));
    }

    /// Register `RINIT` hook.
    pub fn on_request_init(&mut self, func: impl Fn(&ModuleEntry) -> bool + Send + Sync + 'static) {
        self.request_init = Some(Box::new(func));
    }

    /// Register `RSHUTDOWN` hook.
    pub fn on_request_shutdown(
        &mut self, func: impl Fn(&ModuleEntry) -> bool + Send + Sync + 'static,
    ) {
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
        assert!(!self.name.as_bytes().is_empty(), "module name must be set");
        assert!(
            !self.version.as_bytes().is_empty(),
            "module version must be set"
        );

        let module = Box::new(self);

        let mut entry: Box<zend_module_entry> = Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: null(),
            functions: module.function_entries(),
            module_startup_func: Some(module_startup),
            module_shutdown_func: Some(module_shutdown),
            request_startup_func: Some(request_startup),
            request_shutdown_func: Some(request_shutdown),
            info_func: Some(module_info),
            version: null(),
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

        // Hide module pointer after name.
        let mut name: Vec<u8> =
            Vec::with_capacity(module.name.as_bytes_with_nul().len() + size_of::<usize>());
        name.extend_from_slice(module.name.as_bytes_with_nul());
        name.extend_from_slice(&(Box::into_raw(module) as usize).to_le_bytes());
        entry.name = ManuallyDrop::new(name).as_ptr().cast();

        Box::into_raw(entry)
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

/// Wrapper of [`zend_module_entry`](crate::sys::zend_module_entry).
#[repr(transparent)]
pub struct ModuleEntry {
    inner: zend_module_entry,
    _p: PhantomData<*mut ()>,
}

impl ModuleEntry {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const zend_module_entry) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    unsafe fn from_mut_ptr<'a>(ptr: *mut zend_module_entry) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_module_entry) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_module_entry {
        &self.inner
    }

    /// Get the name of module.
    pub fn name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(self.inner.name)
        }
    }

    #[inline]
    fn from_globals(module_number: c_int) -> &'static Self {
        Self::from_globals_mut(module_number)
    }

    fn from_globals_mut(module_number: c_int) -> &'static mut Self {
        unsafe {
            for (_, ptr) in ZArr::from_mut_ptr(&mut module_registry).iter_mut() {
                let module = phper_z_ptr_p(ptr.as_ptr()) as *mut zend_module_entry;
                dbg!(CStr::from_ptr((*module).name).to_str());
                if (*module).module_number == module_number {
                    return Self::from_mut_ptr(module);
                }
            }
            panic!("Find module_entry from module_number failed");
        }
    }

    #[inline]
    unsafe fn get_module(&self) -> &Module {
        Self::inner_get_module((*self.as_ptr()).name as *mut u8)
    }

    #[inline]
    unsafe fn get_module_mut(&mut self) -> &mut Module {
        Self::inner_get_module((*self.as_ptr()).name as *mut u8)
    }

    unsafe fn inner_get_module<'a>(mut ptr: *mut u8) -> &'a mut Module {
        while *ptr != 0 {
            ptr = ptr.offset(1);
        }
        let mut buf = [0u8; size_of::<usize>()];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = *ptr.add(i + 1);
        }
        let module = usize::from_le_bytes(buf) as *mut Module;
        module.as_mut().unwrap()
    }
}
