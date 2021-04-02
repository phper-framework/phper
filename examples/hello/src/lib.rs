use phper::{
    c_str_ptr, php_function, php_get_module, php_minfo_function, php_minit_function,
    php_mshutdown_function, php_rinit_function, php_rshutdown_function,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, PHP_INI_SYSTEM,
    },
    zend::{
        api::{FunctionEntries, ModuleGlobals},
        compile::{create_zend_arg_info, MultiInternalArgInfo},
        ini::IniEntries,
        modules::{ModuleArgs, ModuleEntry, ModuleEntryBuilder},
        types::{ExecuteData, SetVal},
    },
};

static SIMPLE_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);

static INI_ENTRIES: IniEntries<1> = IniEntries::new([SIMPLE_ENABLE.create_ini_entry(
    "hello.enable",
    "1",
    Some(OnUpdateBool),
    PHP_INI_SYSTEM,
)]);

#[php_minit_function]
fn module_init(args: ModuleArgs) -> bool {
    args.register_ini_entries(&INI_ENTRIES);
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
        php_info_print_table_row(2, c_str_ptr!("hello.version"), (*module.as_ptr()).version);
        php_info_print_table_row(
            2,
            c_str_ptr!("hello.enable"),
            if SIMPLE_ENABLE.get() {
                c_str_ptr!("1")
            } else {
                c_str_ptr!("0")
            },
        );
        php_info_print_table_end();
    }
}

#[php_function]
pub fn say_hello(execute_data: &mut ExecuteData) -> impl SetVal {
    execute_data
        .parse_parameters::<&str>()
        .map(|name| format!("Hello, {}!", name))
}

static ARG_INFO_SAY_HELLO: MultiInternalArgInfo<1> =
    MultiInternalArgInfo::new(1, false, [create_zend_arg_info(c_str_ptr!("name"), false)]);

static FUNCTION_ENTRIES: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
    fname: c_str_ptr!("say_hello"),
    handler: Some(say_hello),
    arg_info: ARG_INFO_SAY_HELLO.as_ptr(),
    num_args: 2,
    flags: 0,
}]);

static MODULE_ENTRY: ModuleEntry = ModuleEntryBuilder::new(
    c_str_ptr!(env!("CARGO_PKG_NAME")),
    c_str_ptr!(env!("CARGO_PKG_VERSION")),
)
.functions(FUNCTION_ENTRIES.as_ptr())
.module_startup_func(module_init)
.module_shutdown_func(module_shutdown)
.request_startup_func(request_init)
.request_shutdown_func(request_shutdown)
.info_func(module_info)
.build();

#[php_get_module]
pub fn get_module() -> &'static ModuleEntry {
    &MODULE_ENTRY
}
