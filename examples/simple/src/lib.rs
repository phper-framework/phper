use phper::{c_str_ptr, php_fn, ebox};
use phper::sys::{ZEND_RESULT_CODE_SUCCESS, zend_parse_parameters, zend_internal_arg_info, zend_function_entry, PHP_INI_SYSTEM};
use phper::sys::{zend_ini_entry_def, zend_module_entry, zend_register_ini_entries, zend_unregister_ini_entries, OnUpdateBool, phper_zval_string};
use phper::sys::{OnUpdateString, zend_class_entry, zend_register_internal_class, zend_declare_property_string, ZEND_ACC_PUBLIC, zend_type, zend_read_property};
use phper::zend::api::{FunctionEntries, ModuleGlobals, function_entry_end};
use phper::zend::compile::{InternalArgInfos, internal_arg_info_begin};
use phper::zend::ini::{IniEntryDefs, ini_entry_def_end};
use phper::zend::modules::{ModuleEntry, create_zend_module_entry};
use phper::zend::types::{ExecuteData, Val, SetVal, Value};
use phper::{
    php_function, php_minit, php_minit_function, php_mshutdown, php_mshutdown_function,
    php_rinit_function, php_rshutdown_function,
};
use phper::{php_minfo, php_minfo_function, php_rinit, php_rshutdown, zend_get_module};
use std::ffi::{CStr, CString};
use std::mem;
use std::mem::{size_of, transmute};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort};
use std::ptr::{null, null_mut};
use phper::zend::exceptions::MyException;
use phper::sys::{php_info_print_table_start, php_info_print_table_row, php_info_print_table_end, phper_init_class_entry};

static mut MY_CLASS_CE: *mut zend_class_entry = null_mut();

static SIMPLE_ENABLE: ModuleGlobals<bool> = ModuleGlobals::new(false);
static SIMPLE_TEXT: ModuleGlobals<*const c_char> = ModuleGlobals::new(null());

static INI_ENTRIES: IniEntryDefs<3> = IniEntryDefs::new([
    SIMPLE_ENABLE.create_ini_entry_def("simple.enable", "1", Some(OnUpdateBool), PHP_INI_SYSTEM),
    SIMPLE_TEXT.create_ini_entry_def("simple.text", "", Some(OnUpdateString), PHP_INI_SYSTEM),
    ini_entry_def_end(),
]);

#[php_minit_function]
fn m_init_simple(type_: c_int, module_number: c_int) -> bool {
    unsafe {
        zend_register_ini_entries(INI_ENTRIES.as_ptr(), module_number);
    }
    unsafe {
        let mut my_class_ce = phper_init_class_entry(c_str_ptr!("MyClass"), MY_CLASS_METHODS.as_ptr());
        MY_CLASS_CE = zend_register_internal_class(&mut my_class_ce);
        zend_declare_property_string(MY_CLASS_CE, c_str_ptr!("foo"), 3, c_str_ptr!("bar"), ZEND_ACC_PUBLIC as c_int);
    }
    true
}

#[php_mshutdown_function]
fn m_shutdown_simple(type_: c_int, module_number: c_int) -> bool {
    unsafe {
        zend_unregister_ini_entries(module_number);
    }
    true
}

#[php_rinit_function]
fn r_init_simple(type_: c_int, module_number: c_int) -> bool {
    true
}

#[php_rshutdown_function]
fn r_shutdown_simple(type_: c_int, module_number: c_int) -> bool {
    true
}

#[php_minfo_function]
fn m_info_simple(zend_module: *mut ::phper::sys::zend_module_entry) {
    unsafe {
        php_info_print_table_start();
        php_info_print_table_row(2, c_str_ptr!("simple.enable"), format!("{}\0", *SIMPLE_ENABLE.as_ptr()).as_ptr());
        php_info_print_table_row(2, c_str_ptr!("simple.text"), format!("{}\0", CStr::from_ptr((*SIMPLE_TEXT.as_ptr())).to_str().unwrap()).as_ptr());
        php_info_print_table_end();
    }
}

#[php_function]
pub fn test_simple(execute_data: ExecuteData) -> impl SetVal {
    let mut a: *const c_char = null_mut();
    let mut a_len = 0;
    let mut b: *const c_char = null_mut();
    let mut b_len = 0;

    unsafe {
        if zend_parse_parameters(
            execute_data.num_args() as c_int,
            c_str_ptr!("ss"),
            &mut a,
            &mut a_len,
            &mut b,
            &mut b_len,
        ) != ZEND_RESULT_CODE_SUCCESS
        {
            return None;
        }

        let s = CStr::from_ptr((*SIMPLE_TEXT.as_ptr())).to_str().unwrap();
        println!("simple.text: '{}'", s);

        Some(format!(
            "(a . b) = {}{}",
            CStr::from_ptr(a).to_str().unwrap(),
            CStr::from_ptr(b).to_str().unwrap(),
        ))
    }
}

static ARG_INFO_TEST_SIMPLE: InternalArgInfos<3> = InternalArgInfos::new([
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


static ARG_INFO_MY_CLASS_FOO: InternalArgInfos<2> = InternalArgInfos::new([
    zend_internal_arg_info {
        name: 2 as *const _,
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
    let mut prefix: *const c_char = null_mut();
    let mut prefix_len = 0;

    unsafe {
        if zend_parse_parameters(
            execute_data.num_args() as c_int,
            c_str_ptr!("s"),
            &mut prefix,
            &mut prefix_len,
        ) != ZEND_RESULT_CODE_SUCCESS
        {
            return None;
        }

        let prefix = CStr::from_ptr(prefix).to_str().unwrap();

        let this = if execute_data.get_type() == phper::sys::IS_OBJECT as zend_type {
            execute_data.get_this()
        } else {
            null_mut()
        };

        let foo = zend_read_property(MY_CLASS_CE, this, c_str_ptr!("foo"), 3, 1, null_mut());
        let foo = Val::from_raw(foo);
        let foo = foo.as_c_str().unwrap().to_str().unwrap();
        Some(format!("{}{}", prefix, foo))
    }
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
