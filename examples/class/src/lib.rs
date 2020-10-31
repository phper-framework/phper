#![feature(allocator_api)]

use phper::{c_str_ptr, php_fn, ebox};
use phper::sys::{ZEND_RESULT_CODE_SUCCESS, zend_parse_parameters, zend_internal_arg_info, zend_function_entry, PHP_INI_SYSTEM, ZEND_ACC_PUBLIC};
use phper::sys::{zend_ini_entry_def, zend_module_entry, zend_register_ini_entries, zend_unregister_ini_entries, zend_class_entry};
use phper::zend::api::FunctionEntries;
use phper::zend::compile::InternalArgInfos;
use phper::zend::ini::IniEntryDefs;
use phper::zend::modules::ModuleEntry;
use phper::zend::types::{ExecuteData, Val};
use phper::{
    php_function, php_minit, php_minit_function, php_mshutdown, php_mshutdown_function,
    php_rinit_function, php_rshutdown_function,
};
use phper::{php_minfo, php_minfo_function, php_rinit, php_rshutdown, zend_get_module};
use std::ffi::{CStr, CString};
use std::mem;
use std::mem::{size_of, transmute, MaybeUninit, size_of_val, zeroed};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort};
use std::ptr::{null, null_mut};
use phper::sys::{zend_string, zend_register_internal_class_ex, phper_init_class_entry, zend_register_internal_class, phper_zval_string};
use phper::sys::{zend_declare_property_string, zend_type, zend_read_property};

static mut MY_CLASS_CE: *mut zend_class_entry = null_mut();

static INI_ENTRIES: IniEntryDefs<1> = IniEntryDefs::new([
    unsafe { transmute([0u8; size_of::<zend_ini_entry_def>()]) },
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
pub fn my_class_foo(execute_data: ExecuteData, return_value: Val) {
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
            return;
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
        let s = CString::new(&*format!("{}{}", prefix, foo)).unwrap();

        phper_zval_string(return_value.get(), s.as_ptr());
    }
}

#[php_minit_function]
fn m_init_simple(type_: c_int, module_number: c_int) -> bool {
    unsafe {
        zend_register_ini_entries(INI_ENTRIES.get(), module_number);
    }

    unsafe {
        let mut my_class_ce = phper_init_class_entry(c_str_ptr!("MyClass"), MY_CLASS_METHODS.get());
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
}

static MODULE_ENTRY: ModuleEntry = ModuleEntry::new(zend_module_entry {
    size: size_of::<zend_module_entry>() as c_ushort,
    zend_api: phper::sys::ZEND_MODULE_API_NO as c_uint,
    zend_debug: phper::sys::ZEND_DEBUG as c_uchar,
    zts: phper::sys::USING_ZTS as c_uchar,
    ini_entry: std::ptr::null(),
    deps: std::ptr::null(),
    name: c_str_ptr!(env!("CARGO_PKG_NAME")),
    functions: null(),
    module_startup_func: Some(php_minit!(m_init_simple)),
    module_shutdown_func: Some(php_mshutdown!(m_shutdown_simple)),
    request_startup_func: Some(php_rinit!(r_init_simple)),
    request_shutdown_func: Some(php_rshutdown!(r_shutdown_simple)),
    info_func: Some(php_minfo!(m_info_simple)),
    version: c_str_ptr!(env!("CARGO_PKG_VERSION")),
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
    build_id: phper::sys::PHP_MODULE_BUILD_ID,
});

#[zend_get_module]
pub fn get_module() -> &'static ModuleEntry {
    &MODULE_ENTRY
}
