use crate::{
    c_str_ptr,
    classes::{Class, ClassEntity, StdClass},
    functions::{Argument, Callable, Function, FunctionEntity},
    ini::{IniEntity, IniValue, Policy, StrPtrBox},
    sys::*,
    EXCEPTION_CLASS_NAME,
};
use once_cell::sync::Lazy;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    mem::{size_of, zeroed},
    os::raw::{c_int, c_uchar, c_uint, c_ushort},
    ptr::{null, null_mut},
    sync::RwLock,
};

static GLOBAL_MODULE: Lazy<RwLock<Module>> = Lazy::new(Default::default);

#[doc(hidden)]
pub fn read_global_module<R>(f: impl FnOnce(&Module) -> R) -> R {
    let module = (&*GLOBAL_MODULE).read().expect("get write lock failed");
    f(&module)
}

#[doc(hidden)]
pub fn write_global_module<R>(f: impl FnOnce(&mut Module) -> R) -> R {
    let mut module = (&*GLOBAL_MODULE).write().expect("get write lock failed");
    f(&mut module)
}

unsafe extern "C" fn module_startup(r#type: c_int, module_number: c_int) -> c_int {
    let args = ModuleArgs::new(r#type, module_number);
    write_global_module(|module| {
        args.register_ini_entries(module.ini_entries());
        for (_, class_entity) in &mut module.class_entities {
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
    pub(crate) class_entities: HashMap<String, ClassEntity>,
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

    pub fn add_function(
        &mut self,
        name: impl ToString,
        handler: impl Function + 'static,
        arguments: Vec<Argument>,
    ) {
        self.function_entities.push(FunctionEntity::new(
            name,
            Callable::Function(Box::new(handler)),
            arguments,
        ));
    }

    pub fn add_class(&mut self, name: impl ToString, class: impl Class + 'static) {
        self.class_entities
            .insert(name.to_string(), unsafe { ClassEntity::new(name, class) });
    }

    pub unsafe fn module_entry(&mut self) -> *const zend_module_entry {
        assert!(!self.name.is_empty(), "module name must be set");
        assert!(!self.version.is_empty(), "module version must be set");

        self.add_error_exception_class();

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
            entries.push(unsafe { f.entry() });
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

        entries.push(zeroed::<zend_ini_entry_def>());

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

    fn add_error_exception_class(&mut self) {
        let mut class = StdClass::new();
        class.extends("\\Exception");
        self.add_class(EXCEPTION_CLASS_NAME, class);
    }
}

pub struct ModuleArgs {
    #[allow(dead_code)]
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
