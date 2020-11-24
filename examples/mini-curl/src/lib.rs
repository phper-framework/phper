use phper::{
    c_str_ptr, php_fn, php_function, php_minfo, php_minfo_function, php_minit, php_minit_function,
    php_mshutdown, php_mshutdown_function, php_rinit, php_rinit_function, php_rshutdown,
    php_rshutdown_function,
    sys::{php_info_print_table_end, php_info_print_table_start, PHP_INI_SYSTEM},
    zend::{
        api::{FunctionEntries, FunctionEntryBuilder},
        compile::{create_zend_arg_info, MultiInternalArgInfo},
        ini::{create_ini_entry, IniEntries},
        modules::{create_zend_module_entry, ModuleArgs, ModuleEntry},
        types::{ClassEntry, ExecuteData, ReturnValue, SetVal},
    },
    zend_get_module,
};
use std::ptr::null;

static MINI_CURL_CE: ClassEntry = ClassEntry::new();

static INI_ENTRIES: IniEntries<1> =
    IniEntries::new([create_ini_entry("curl.cainfo", "", PHP_INI_SYSTEM)]);

#[php_minit_function]
fn module_init(args: ModuleArgs) -> bool {
    args.register_ini_entries(&INI_ENTRIES);
    MINI_CURL_CE.init("MiniCurl", &MINI_CURL_METHODS);
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
fn module_info(__module: &ModuleEntry) {
    unsafe {
        php_info_print_table_start();
        php_info_print_table_end();
    }
}

static ARG_INFO_VOID: MultiInternalArgInfo<0> = MultiInternalArgInfo::new(0, false, []);

static ARG_INFO_MINI_CURL_CONSTRUCT: MultiInternalArgInfo<1> =
    MultiInternalArgInfo::new(0, false, [create_zend_arg_info(c_str_ptr!("url"), false)]);

static MINI_CURL_METHODS: FunctionEntries<2> = FunctionEntries::new([
    FunctionEntryBuilder::new(
        c_str_ptr!("__construct"),
        Some(php_fn!(mini_curl_construct)),
    )
    .arg_info(&ARG_INFO_MINI_CURL_CONSTRUCT)
    .build(),
    FunctionEntryBuilder::new(c_str_ptr!("__destruct"), Some(php_fn!(mini_curl_destruct)))
        .arg_info(&ARG_INFO_VOID)
        .build(),
]);

#[php_function]
pub fn mini_curl_construct(execute_data: &mut ExecuteData) -> impl SetVal {
    match execute_data
        .parse_parameters_optional("")
        .map(|_url: &str| {})
    {
        Some(_) => ReturnValue::Bool(true),
        None => ReturnValue::Bool(false),
    }
}

#[php_function]
pub fn mini_curl_destruct(execute_data: &mut ExecuteData) -> impl SetVal {
    if execute_data.parse_parameters::<()>().is_none() {
        return ReturnValue::Bool(false);
    }

    ReturnValue::Null
}

static MODULE_ENTRY: ModuleEntry = ModuleEntry::new(create_zend_module_entry(
    c_str_ptr!(env!("CARGO_PKG_NAME")),
    c_str_ptr!(env!("CARGO_PKG_VERSION")),
    null(),
    Some(php_minit!(module_init)),
    Some(php_mshutdown!(module_shutdown)),
    Some(php_rinit!(request_init)),
    Some(php_rshutdown!(request_shutdown)),
    Some(php_minfo!(module_info)),
    None,
    None,
));

#[zend_get_module]
pub fn get_module() -> &'static ModuleEntry {
    &MODULE_ENTRY
}
