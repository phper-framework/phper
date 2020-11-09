use phper::sys::OnUpdateBool;
use phper::sys::{php_info_print_table_end, php_info_print_table_row, php_info_print_table_start};
use phper::sys::{zend_function_entry, zend_internal_arg_info, PHP_INI_SYSTEM};
use phper::sys::{zend_read_property, zend_type, OnUpdateString, ZEND_ACC_PUBLIC};
use phper::zend::api::{function_entry_end, FunctionEntries, ModuleGlobals};
use phper::zend::compile::{internal_arg_info_begin, MultiInternalArgInfo};
use phper::zend::ini::{ini_entry_def_end, IniEntryDefs};
use phper::zend::modules::{create_zend_module_entry, ModuleArgs, ModuleEntry};
use phper::zend::types::{ClassEntry, ExecuteData, SetVal, Val};
use phper::{c_str_ptr, php_fn};
use phper::{
    php_function, php_minit, php_minit_function, php_mshutdown, php_mshutdown_function,
    php_rinit_function, php_rshutdown_function,
};
use phper::{php_minfo, php_minfo_function, php_rinit, php_rshutdown, zend_get_module};
use std::mem::{size_of, transmute};
use std::os::raw::c_char;
use std::ptr::{null, null_mut};

static MY_CLASS_CE: ClassEntry = ClassEntry::new();

static SIMPLE_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);
static SIMPLE_TEXT: ModuleGlobals<*const c_char> = ModuleGlobals::new(null());

static INI_ENTRIES: IniEntryDefs<3> = IniEntryDefs::new([
    SIMPLE_ENABLE.create_ini_entry_def("simple.enable", "1", Some(OnUpdateBool), PHP_INI_SYSTEM),
    SIMPLE_TEXT.create_ini_entry_def("simple.text", "", Some(OnUpdateString), PHP_INI_SYSTEM),
    ini_entry_def_end(),
]);

#[php_minit_function]
fn m_init_simple(args: ModuleArgs) -> bool {
    args.register_ini_entries(&INI_ENTRIES);
    MY_CLASS_CE.init(c_str_ptr!("MyClass"), &MY_CLASS_METHODS);
    MY_CLASS_CE.declare_property("foo", 3, ZEND_ACC_PUBLIC);
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
pub fn test_simple(execute_data: ExecuteData) -> impl SetVal {
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

static ARG_INFO_TEST_SIMPLE: MultiInternalArgInfo<3> = MultiInternalArgInfo::new([
    internal_arg_info_begin(2, false),
    zend_internal_arg_info {
        name: c_str_ptr!("a"),
        type_: 0,
        pass_by_reference: 0,
        is_variadic: 0,
    },
    zend_internal_arg_info {
        name: c_str_ptr!("b"),
        type_: 0,
        pass_by_reference: 0,
        is_variadic: 0,
    },
]);

static FUNCTION_ENTRIES: FunctionEntries<2> = FunctionEntries::new([
    zend_function_entry {
        fname: c_str_ptr!("test_simple"),
        handler: Some(php_fn!(test_simple)),
        arg_info: ARG_INFO_TEST_SIMPLE.get(),
        num_args: 2,
        flags: 0,
    },
    function_entry_end(),
]);

static ARG_INFO_MY_CLASS_FOO: MultiInternalArgInfo<2> = MultiInternalArgInfo::new([
    zend_internal_arg_info {
        name: 1 as *const _,
        type_: 0,
        pass_by_reference: 0,
        is_variadic: 0,
    },
    zend_internal_arg_info {
        name: c_str_ptr!("prefix"),
        type_: 0,
        pass_by_reference: 0,
        is_variadic: 0,
    },
]);

static MY_CLASS_METHODS: FunctionEntries<2> = FunctionEntries::new([
    zend_function_entry {
        fname: c_str_ptr!("foo"),
        handler: Some(php_fn!(my_class_foo)),
        arg_info: ARG_INFO_MY_CLASS_FOO.get(),
        num_args: 1,
        flags: 0,
    },
    unsafe { transmute([0u8; size_of::<zend_function_entry>()]) },
]);

#[php_function]
pub fn my_class_foo(execute_data: ExecuteData) -> impl SetVal {
    execute_data.parse_parameters::<&str>().map(|prefix| {
        let this = if execute_data.get_type() == phper::sys::IS_OBJECT as zend_type {
            execute_data.get_this()
        } else {
            null_mut()
        };

        let foo = unsafe {
            zend_read_property(MY_CLASS_CE.get(), this, c_str_ptr!("foo"), 3, 1, null_mut())
        };
        let foo = Val::from_raw(foo);
        let foo = foo.as_c_str().unwrap().to_str().unwrap();
        format!("{}{}", prefix, foo)
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
