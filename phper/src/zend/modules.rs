use crate::{
    sys::{
        zend_class_entry, zend_function_entry, zend_ini_entry_def, zend_internal_arg_info,
        zend_module_entry, zend_register_ini_entries, zend_string, zend_unregister_ini_entries,
        PHP_MODULE_BUILD_ID, USING_ZTS, ZEND_DEBUG, ZEND_MODULE_API_NO,
    },
    zend::{
        api::{invoke, Function, FunctionEntity},
        classes::{Class, ClassEntity, ClassEntry},
        ini::{IniEntity, IniEntries, Policy},
    },
};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell, RefMut},
    collections::HashMap,
    mem::{forget, size_of, transmute, zeroed},
    ops::DerefMut,
    os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void},
    ptr::{null, null_mut},
    sync::{atomic::AtomicPtr, Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
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
    r#type: c_int,
    module_number: c_int,
}

impl ModuleArgs {
    pub const fn new(r#type: c_int, module_number: c_int) -> Self {
        Self {
            r#type,
            module_number,
        }
    }

    pub fn register_ini_entries(&self, ini_entries: *const zend_ini_entry_def) {
        unsafe {
            zend_register_ini_entries(ini_entries, self.module_number);
        }
    }

    pub(crate) fn unregister_ini_entries(&self) {
        unsafe {
            zend_unregister_ini_entries(self.module_number);
        }
    }
}

static GLOBAL_MODULE: Lazy<RwLock<Module>> = Lazy::new(Default::default);

pub fn read_global_module() -> RwLockReadGuard<'static, Module> {
    (&*GLOBAL_MODULE).read().expect("get write lock failed")
}

pub fn write_global_module() -> RwLockWriteGuard<'static, Module> {
    (&*GLOBAL_MODULE).write().expect("get write lock failed")
}

unsafe extern "C" fn module_startup(r#type: c_int, module_number: c_int) -> c_int {
    let args = ModuleArgs::new(r#type, module_number);
    {
        args.register_ini_entries(read_global_module().ini_entries());
    }
    {
        for class_entity in &read_global_module().class_entities {
            class_entity.init();
            class_entity.declare_properties();
        }
    }
    match &read_global_module().module_init {
        Some(f) => f(args) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn module_shutdown(r#type: c_int, module_number: c_int) -> c_int {
    let args = ModuleArgs::new(r#type, module_number);
    args.unregister_ini_entries();

    match &read_global_module().module_shutdown {
        Some(f) => f(args) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn request_startup(r#type: c_int, request_number: c_int) -> c_int {
    match &read_global_module().request_init {
        Some(f) => f(ModuleArgs::new(r#type, request_number)) as c_int,
        None => 1,
    }
}

unsafe extern "C" fn request_shutdown(r#type: c_int, request_number: c_int) -> c_int {
    match &read_global_module().request_shutdown {
        Some(f) => f(ModuleArgs::new(r#type, request_number)) as c_int,
        None => 1,
    }
}

#[derive(Default)]
pub struct Module {
    name: String,
    version: String,
    module_init: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    module_shutdown: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    request_init: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    request_shutdown: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    ini_entities: HashMap<String, IniEntity>,
    function_entities: Vec<FunctionEntity>,
    class_entities: Vec<ClassEntity>,
}

impl Module {
    pub fn set_name(&mut self, name: impl ToString) {
        let mut name = name.to_string();
        name.push('\0');
        self.name = name;
    }

    pub fn set_version(&mut self, version: impl ToString) {
        let mut version = version.to_string();
        version.push('\0');
        self.version = version;
    }

    pub fn on_module_init(&mut self, func: impl Fn(ModuleArgs) -> bool + Send + Sync + 'static) {
        self.module_init = Some(Box::new(func));
    }

    pub fn on_module_shutdown(
        &mut self,
        func: impl Fn(ModuleArgs) -> bool + Send + Sync + 'static,
    ) {
        self.module_shutdown = Some(Box::new(func));
    }

    pub fn on_request_init(&mut self, func: impl Fn(ModuleArgs) -> bool + Send + Sync + 'static) {
        self.request_init = Some(Box::new(func));
    }

    pub fn on_request_shutdown(
        &mut self,
        func: impl Fn(ModuleArgs) -> bool + Send + Sync + 'static,
    ) {
        self.request_shutdown = Some(Box::new(func));
    }

    pub fn add_ini(&mut self, name: impl ToString, value: impl ToString, policy: Policy) {
        self.ini_entities
            .insert(name.to_string(), IniEntity::new(name, value, policy));
    }

    pub fn add_function(&mut self, name: impl ToString, handler: impl Function + 'static) {
        let mut name = name.to_string();
        name.push('\0');

        self.function_entities.push(FunctionEntity {
            name,
            handler: Box::new(handler),
        });
    }

    pub fn add_class(&mut self, name: impl ToString, class: impl Class + 'static) {
        self.class_entities
            .push(unsafe { ClassEntity::new(name, class) })
    }

    pub unsafe fn module_entry(&self) -> *const zend_module_entry {
        assert!(!self.name.is_empty(), "module name must be set");
        assert!(!self.version.is_empty(), "module version must be set");

        let entry: Box<zend_module_entry> = Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: self.name.as_ptr().cast(),
            functions: self.function_entries(),
            module_startup_func: Some(module_startup),
            module_shutdown_func: Some(module_shutdown),
            request_startup_func: Some(request_startup),
            request_shutdown_func: Some(request_shutdown),
            info_func: None,
            version: null(),
            globals_size: 0usize,
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
            build_id: PHP_MODULE_BUILD_ID,
        });

        Box::into_raw(entry)
    }

    fn function_entries(&self) -> *const zend_function_entry {
        if self.function_entities.is_empty() {
            return null();
        }

        let mut entries = Vec::new();

        for f in &self.function_entities {
            let mut infos = Vec::new();
            infos.push(unsafe { zeroed::<zend_internal_arg_info>() });

            let mut last_arg_info = unsafe { zeroed::<zend_internal_arg_info>() };
            last_arg_info.name = ((&f.handler) as *const _ as *mut i8).cast();
            infos.push(last_arg_info);

            let entry = zend_function_entry {
                fname: f.name.as_ptr().cast(),
                handler: Some(invoke),
                arg_info: Box::into_raw(infos.into_boxed_slice()).cast(),
                num_args: 0,
                flags: 0,
            };

            entries.push(entry);
        }

        entries.push(unsafe { zeroed::<zend_function_entry>() });

        Box::into_raw(entries.into_boxed_slice()).cast()
    }

    fn ini_entries(&self) -> *const zend_ini_entry_def {
        let mut entries = Vec::new();

        for (_, ini) in &self.ini_entities {
            ini.ini_entry_def();
        }

        entries.push(unsafe { zeroed::<zend_ini_entry_def>() });

        Box::into_raw(entries.into_boxed_slice()).cast()
    }
}
