use phper::{
    c_str_ptr, php_fn, php_function, php_minfo, php_minfo_function, php_minit, php_minit_function,
    php_mshutdown, php_mshutdown_function, php_rinit, php_rinit_function, php_rshutdown,
    php_rshutdown_function,
    sys::{
        php_info_print_table_end, php_info_print_table_row, php_info_print_table_start,
        zend_function_entry, OnUpdateBool, OnUpdateString, PHP_INI_SYSTEM, ZEND_ACC_PUBLIC,
    },
    zend::{
        api::{FunctionEntries, ModuleGlobals},
        compile::{create_zend_arg_info, MultiInternalArgInfo},
        ini::IniEntries,
        modules::{create_zend_module_entry, ModuleArgs, ModuleEntry},
        types::{ClassEntry, ExecuteData, SetVal, Value},
    },
    zend_get_module,
};
use std::{
    os::raw::{c_char, c_int},
    ptr::null,
};

static MY_CLASS_CE: ClassEntry = ClassEntry::new();

static SIMPLE_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);
static SIMPLE_TEXT: ModuleGlobals<*const c_char> = ModuleGlobals::new(null());

static INI_ENTRIES: IniEntries<2> = IniEntries::new([
    SIMPLE_ENABLE.create_ini_entry_def("simple.enable", "1", Some(OnUpdateBool), PHP_INI_SYSTEM),
    SIMPLE_TEXT.create_ini_entry_def("simple.text", "", Some(OnUpdateString), PHP_INI_SYSTEM),
]);

#[php_minit_function]
fn m_init_simple(args: ModuleArgs) -> bool {
    args.register_ini_entries(&INI_ENTRIES);
    MY_CLASS_CE.init(c_str_ptr!("MyClass"), &MY_CLASS_METHODS);
    MY_CLASS_CE.declare_property("name", "world", ZEND_ACC_PUBLIC as c_int);
    true
}

#[php_mshutdown_function]
fn m_shutdown_simple(args: ModuleArgs) -> bool {
    args.unregister_ini_entries();
    true
}

#[php_rinit_function]
fn r_init_simple(_args: ModuleArgs) -> bool {
    true
}

#[php_rshutdown_function]
fn r_shutdown_simple(_args: ModuleArgs) -> bool {
    true
}

#[php_minfo_function]
fn m_info_simple(module: &ModuleEntry) {
    unsafe {
        php_info_print_table_start();
        php_info_print_table_row(2, c_str_ptr!("simple.version"), (*module.as_ptr()).version);
        php_info_print_table_row(
            2,
            c_str_ptr!("simple.build_id"),
            (*module.as_ptr()).build_id,
        );
        php_info_print_table_row(
            2,
            c_str_ptr!("simple.enable"),
            if SIMPLE_ENABLE.get() {
                c_str_ptr!("1")
            } else {
                c_str_ptr!("0")
            },
        );
        php_info_print_table_row(2, c_str_ptr!("simple.text"), SIMPLE_TEXT.get());
        php_info_print_table_end();
    }
}

#[php_function]
pub fn test_simple(execute_data: &mut ExecuteData) -> impl SetVal {
    execute_data
        .parse_parameters::<(&str, &str)>()
        .map(|(a, b)| {
            format!(
                "a = {}, a_len = {}, b = {}, b_len = {}",
                a,
                a.len(),
                b,
                b.len(),
            )
        })
}

static ARG_INFO_TEST_SIMPLE: MultiInternalArgInfo<2> = MultiInternalArgInfo::new(
    [
        create_zend_arg_info(c_str_ptr!("a"), false),
        create_zend_arg_info(c_str_ptr!("b"), false),
    ],
    false,
);

static FUNCTION_ENTRIES: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
    fname: c_str_ptr!("test_simple"),
    handler: Some(php_fn!(test_simple)),
    arg_info: ARG_INFO_TEST_SIMPLE.as_ptr(),
    num_args: 2,
    flags: 0,
}]);

static ARG_INFO_MY_CLASS_HELLO: MultiInternalArgInfo<1> =
    MultiInternalArgInfo::new([create_zend_arg_info(c_str_ptr!("prefix"), false)], false);

static MY_CLASS_METHODS: FunctionEntries<1> = FunctionEntries::new([zend_function_entry {
    fname: c_str_ptr!("hello"),
    handler: Some(php_fn!(my_class_hello)),
    arg_info: ARG_INFO_MY_CLASS_HELLO.as_ptr(),
    num_args: 1,
    flags: 0,
}]);

#[php_function]
pub fn my_class_hello(execute_data: &mut ExecuteData) -> impl SetVal {
    execute_data.parse_parameters::<&str>().map(|prefix| {
        let this = execute_data.get_this();
        let val = MY_CLASS_CE.read_property(this, "name");
        let value = val.try_into_value().unwrap();

        if let Value::CStr(foo) = value {
            Some(format!("{}{}", prefix, foo.to_str().unwrap()))
        } else {
            None
        }
    })
}

static MODULE_ENTRY: ModuleEntry = ModuleEntry::new(create_zend_module_entry(
    c_str_ptr!(env!("CARGO_PKG_NAME")),
    c_str_ptr!(env!("CARGO_PKG_VERSION")),
    FUNCTION_ENTRIES.as_ptr(),
    Some(php_minit!(m_init_simple)),
    Some(php_mshutdown!(m_shutdown_simple)),
    Some(php_rinit!(r_init_simple)),
    Some(php_rshutdown!(r_shutdown_simple)),
    Some(php_minfo!(m_info_simple)),
    None,
    None,
));

#[zend_get_module]
pub fn get_module() -> &'static ModuleEntry {
    &MODULE_ENTRY
}
