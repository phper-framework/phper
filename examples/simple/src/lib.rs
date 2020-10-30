use phper::{php_function, PHPerResult};
use phper::php_get_module;
use phper::zend::modules::ModuleEntry;
use phper::sys::zend_module_entry;
use std::mem::size_of;
use std::os::raw::{c_ushort, c_uint, c_uchar, c_char, c_int};
use phper::c_str_ptr;
use std::ptr::{null, null_mut};
use phper::zend::types::{ExecuteData, Val};
use phper::zend::api::FunctionEntries;
use phper::php_fn;
use phper::sys::zend_function_entry;
use phper::zend::compile::{InternalArgInfos};
use phper::sys::zend_internal_arg_info;
use phper::sys::zend_parse_parameters;
use phper::sys::ZEND_RESULT_CODE_SUCCESS;
use std::ffi::CStr;

#[php_function]
pub fn test_simple(execute_data: ExecuteData, return_value: Val) {
    println!("zif_test_simple success, num args: {}", execute_data.num_args());

    let mut a: *const c_char = null_mut();
    let mut a_len = 0;
    let mut b: *const c_char = null_mut();
    let mut b_len = 0;

    unsafe {
        if zend_parse_parameters(execute_data.num_args() as c_int, c_str_ptr!("ss"), &mut a, &mut a_len, &mut b, &mut b_len) != ZEND_RESULT_CODE_SUCCESS {
            return;
        }

        println!("echo param, a: {:?}, b: {:?}", CStr::from_ptr(a).to_str(), CStr::from_ptr(b).to_str());
    }
}

#[php_get_module]
pub fn get_module() -> &'static ModuleEntry {
    static ARG_INFO_TEST_SIMPLE: InternalArgInfos<3> = InternalArgInfos::new([
        zend_internal_arg_info {
            name: 2 as *const _,
            type_: 0,
            pass_by_reference: 0,
            is_variadic: 0,
        },
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
            num_args: 0,
            flags: 0,
        },
        zend_function_entry {
            fname: null(),
            handler: None,
            arg_info: null(),
            num_args: 0,
            flags: 0,
        }
    ]);

    static MODULE_ENTRY: ModuleEntry = ModuleEntry::from_raw(zend_module_entry {
        size: size_of::<zend_module_entry>() as c_ushort,
        zend_api: phper::sys::ZEND_MODULE_API_NO as c_uint,
        zend_debug: phper::sys::ZEND_DEBUG as c_uchar,
        zts: phper::sys::USING_ZTS as c_uchar,
        ini_entry: std::ptr::null(),
        deps: std::ptr::null(),
        name: c_str_ptr!(env!("CARGO_PKG_NAME")),
        functions: FUNCTION_ENTRIES.get(),
        module_startup_func: None,
        module_shutdown_func: None,
        request_startup_func: None,
        request_shutdown_func: None,
        info_func: None,
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

    &MODULE_ENTRY
}
