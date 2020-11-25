use phper::{
    c_str_ptr, php_fn, php_function, php_minfo, php_minfo_function, php_minit, php_minit_function,
    php_mshutdown, php_mshutdown_function, php_rinit, php_rinit_function, php_rshutdown,
    php_rshutdown_function,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, OnUpdateString, PHP_INI_SYSTEM,
    },
    zend::{
        api::{FunctionEntries, ModuleGlobals},
        compile::{create_zend_arg_info, MultiInternalArgInfo, Visibility},
        ini::IniEntries,
        modules::{ModuleArgs, ModuleEntry, ModuleEntryBuilder},
        types::{ClassEntry, ExecuteData, SetVal, Value},
    },
    zend_get_module,
};
use std::{os::raw::c_char, ptr::null};

static HELLO_CLASS_CE: ClassEntry = ClassEntry::new();

static HELLO_CLASS_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);
static HELLO_CLASS_DESCRIPTION: ModuleGlobals<*const c_char> = ModuleGlobals::new(null());

static INI_ENTRIES: IniEntries<2> = IniEntries::new([
    HELLO_CLASS_ENABLE.create_ini_entry(
        "hello_class.enable",
        "1",
        Some(OnUpdateBool),
        PHP_INI_SYSTEM,
    ),
    HELLO_CLASS_DESCRIPTION.create_ini_entry(
        "hello_class.description",
        "",
        Some(OnUpdateString),
        PHP_INI_SYSTEM,
    ),
]);

#[php_minit_function]
fn module_init(args: ModuleArgs) -> bool {
    args.register_ini_entries(&INI_ENTRIES);
    HELLO_CLASS_CE.init("HelloClass", &HELLO_CLASS_METHODS);
    HELLO_CLASS_CE.declare_property("name", "world", Visibility::Public);
    true
}

#[php_mshutdown_function]
fn module_shutdown(args: ModuleArgs) -> bool {
    args.unregister_ini_entries();
    true
}

#[php_rinit_function]
fn request_init(_args: ModuleArgs) -> bool {
    true
}

#[php_rshutdown_function]
fn request_shutdown(_args: ModuleArgs) -> bool {
    true
}

#[php_minfo_function]
fn module_info(module: &ModuleEntry) {
    unsafe {
        php_info_print_table_start();
        php_info_print_table_row(
            2,
            c_str_ptr!("hello_class.version"),
            (*module.as_ptr()).version,
        );
        php_info_print_table_row(
            2,
            c_str_ptr!("hello_class.build_id"),
            (*module.as_ptr()).build_id,
        );
        php_info_print_table_row(
            2,
            c_str_ptr!("hello_class.enable"),
            if HELLO_CLASS_ENABLE.get() {
                c_str_ptr!("1")
            } else {
                c_str_ptr!("0")
            },
        );
        php_info_print_table_row(
            2,
            c_str_ptr!("hello_class.description"),
            HELLO_CLASS_DESCRIPTION.get(),
        );
        php_info_print_table_end();
    }
}

static ARG_INFO_HELLO_CLASS_SAY_HELLO: MultiInternalArgInfo<1> = MultiInternalArgInfo::new(
    1,
    false,
    [create_zend_arg_info(c_str_ptr!("prefix"), false)],
);

static HELLO_CLASS_METHODS: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
    fname: c_str_ptr!("sayHello"),
    handler: Some(php_fn!(hello_class_get_hello)),
    arg_info: ARG_INFO_HELLO_CLASS_SAY_HELLO.as_ptr(),
    num_args: 1,
    flags: 0,
}]);

#[php_function]
pub fn hello_class_get_hello(execute_data: &mut ExecuteData) -> impl SetVal {
    execute_data.parse_parameters::<()>().map(|_| {
        let this = execute_data.get_this();
        let val = HELLO_CLASS_CE.read_property(this, "name");
        let value = val.try_into_value().unwrap();

        if let Value::CStr(value) = value {
            Some(format!("Hello, {}!", value.to_str().unwrap()))
        } else {
            None
        }
    })
}

static FUNCTION_ENTRIES: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
    fname: c_str_ptr!("hello_class_get_hello"),
    handler: Some(php_fn!(hello_class_get_hello)),
    arg_info: null(),
    num_args: 0,
    flags: 0,
}]);

static MODULE_ENTRY: ModuleEntry = ModuleEntryBuilder::new(
    c_str_ptr!(env!("CARGO_PKG_NAME")),
    c_str_ptr!(env!("CARGO_PKG_VERSION")),
)
.functions(FUNCTION_ENTRIES.as_ptr())
.module_startup_func(php_minit!(module_init))
.module_shutdown_func(php_mshutdown!(module_shutdown))
.request_startup_func(php_rinit!(request_init))
.request_shutdown_func(php_rshutdown!(request_shutdown))
.info_func(php_minfo!(module_info))
.build();

#[zend_get_module]
pub fn get_module() -> &'static ModuleEntry {
    &MODULE_ENTRY
}
