use crate::{
    c_str_ptr,
    classes::{Class, ClassEntity, ClassEntry},
    functions::{invoke, Function, FunctionEntity},
    ini::{IniEntity, IniValue, Policy, StrPtrBox},
    sys::*,
};
use once_cell::sync::Lazy;
use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell, RefMut},
    collections::HashMap,
    ffi::CStr,
    mem::{forget, size_of, transmute, zeroed},
    ops::DerefMut,
    os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void},
    ptr::{null, null_mut},
    sync::{atomic::AtomicPtr, Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread::LocalKey,
};

static GLOBAL_MODULE: Lazy<RwLock<Module>> = Lazy::new(Default::default);

pub fn read_global_module<R>(f: impl FnOnce(&Module) -> R) -> R {
    let module = (&*GLOBAL_MODULE).read().expect("get write lock failed");
    f(&module)
}

pub fn write_global_module<R>(f: impl FnOnce(&mut Module) -> R) -> R {
    let mut module = (&*GLOBAL_MODULE).write().expect("get write lock failed");
    f(&mut module)
}

unsafe extern "C" fn module_startup(r#type: c_int, module_number: c_int) -> c_int {
    let args = ModuleArgs::new(r#type, module_number);
    read_global_module(|module| {
        args.register_ini_entries(module.ini_entries());
        for class_entity in &module.class_entities {
            class_entity.init();
            class_entity.declare_properties();
        }
        match &module.module_init {
            Some(f) => f(args) as c_int,
            None => 1,
        }
    })
}

unsafe extern "C" fn module_shutdown(r#type: c_int, module_number: c_int) -> c_int {
    let args = ModuleArgs::new(r#type, module_number);
    args.unregister_ini_entries();
    read_global_module(|module| match &module.module_shutdown {
        Some(f) => f(args) as c_int,
        None => 1,
    })
}

unsafe extern "C" fn request_startup(r#type: c_int, request_number: c_int) -> c_int {
    read_global_module(|module| match &module.request_init {
        Some(f) => f(ModuleArgs::new(r#type, request_number)) as c_int,
        None => 1,
    })
}

unsafe extern "C" fn request_shutdown(r#type: c_int, request_number: c_int) -> c_int {
    read_global_module(|module| match &module.request_shutdown {
        Some(f) => f(ModuleArgs::new(r#type, request_number)) as c_int,
        None => 1,
    })
}

unsafe extern "C" fn module_info(zend_module: *mut zend_module_entry) {
    read_global_module(|module| {
        php_info_print_table_start();
        if !module.version.is_empty() {
            php_info_print_table_row(2, c_str_ptr!("version"), module.version.as_ptr());
        }
        if !module.author.is_empty() {
            php_info_print_table_row(2, c_str_ptr!("authors"), module.author.as_ptr());
        }
        php_info_print_table_end();
    });
    display_ini_entries(zend_module);
}

#[derive(Default)]
pub struct Module {
    name: String,
    version: String,
    author: String,
    module_init: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    module_shutdown: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    request_init: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    request_shutdown: Option<Box<dyn Fn(ModuleArgs) -> bool + Send + Sync>>,
    function_entities: Vec<FunctionEntity>,
    class_entities: Vec<ClassEntity>,
}

impl Module {
    thread_local! {
        static BOOL_INI_ENTITIES: RefCell<HashMap<String, IniEntity<bool>>> = Default::default();
        static LONG_INI_ENTITIES: RefCell<HashMap<String, IniEntity<i64>>> = Default::default();
        static REAL_INI_ENTITIES: RefCell<HashMap<String, IniEntity<f64>>> = Default::default();
        static STR_INI_ENTITIES: RefCell<HashMap<String, IniEntity<StrPtrBox>>> = Default::default();
    }

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

    pub fn set_author(&mut self, author: impl ToString) {
        let mut author = author.to_string();
        author.push('\0');
        self.author = author;
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

    pub fn add_bool_ini(&mut self, name: impl ToString, default_value: bool, policy: Policy) {
        Self::BOOL_INI_ENTITIES.with(|entities| {
            entities.borrow_mut().insert(
                name.to_string(),
                IniEntity::new(name, default_value, policy),
            );
        })
    }

    pub fn get_bool_ini(name: &str) -> Option<bool> {
        Self::BOOL_INI_ENTITIES
            .with(|entities| entities.borrow().get(name).map(|entity| *entity.value()))
    }

    pub fn add_long_ini(&mut self, name: impl ToString, default_value: i64, policy: Policy) {
        Self::LONG_INI_ENTITIES.with(|entities| {
            entities.borrow_mut().insert(
                name.to_string(),
                IniEntity::new(name, default_value, policy),
            );
        })
    }

    pub fn get_long_ini(name: &str) -> Option<i64> {
        Self::LONG_INI_ENTITIES
            .with(|entities| entities.borrow().get(name).map(|entity| *entity.value()))
    }

    pub fn add_real_ini(&mut self, name: impl ToString, default_value: f64, policy: Policy) {
        Self::REAL_INI_ENTITIES.with(|entities| {
            entities.borrow_mut().insert(
                name.to_string(),
                IniEntity::new(name, default_value, policy),
            );
        })
    }

    pub fn get_real_ini(name: &str) -> Option<f64> {
        Self::REAL_INI_ENTITIES
            .with(|entities| entities.borrow().get(name).map(|entity| *entity.value()))
    }

    pub fn add_str_ini(
        &mut self,
        name: impl ToString,
        default_value: impl ToString,
        policy: Policy,
    ) {
        Self::STR_INI_ENTITIES.with(|entities| {
            entities.borrow_mut().insert(
                name.to_string(),
                IniEntity::new(name, default_value, policy),
            );
        })
    }

    pub fn get_str_ini(name: &str) -> Option<String> {
        Self::STR_INI_ENTITIES.with(|entities| {
            entities
                .borrow()
                .get(name)
                .and_then(|entity| unsafe { entity.value().to_string() }.ok())
        })
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
            info_func: Some(module_info),
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

    unsafe fn ini_entries(&self) -> *const zend_ini_entry_def {
        let mut entries = Vec::new();

        Self::BOOL_INI_ENTITIES
            .with(|entities| Self::push_ini_entry(&mut entries, &mut *entities.borrow_mut()));
        Self::LONG_INI_ENTITIES
            .with(|entities| Self::push_ini_entry(&mut entries, &mut *entities.borrow_mut()));
        Self::REAL_INI_ENTITIES
            .with(|entities| Self::push_ini_entry(&mut entries, &mut *entities.borrow_mut()));
        Self::STR_INI_ENTITIES
            .with(|entities| Self::push_ini_entry(&mut entries, &mut *entities.borrow_mut()));

        entries.push(unsafe { zeroed::<zend_ini_entry_def>() });

        Box::into_raw(entries.into_boxed_slice()).cast()
    }

    unsafe fn push_ini_entry<T: IniValue>(
        entries: &mut Vec<zend_ini_entry_def>,
        entities: &mut HashMap<String, IniEntity<T>>,
    ) {
        for (_, entry) in &mut *entities.borrow_mut() {
            entries.push(entry.ini_entry_def());
        }
    }
}

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

    pub(crate) fn register_ini_entries(&self, ini_entries: *const zend_ini_entry_def) {
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
